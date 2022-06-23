from typing import TYPE_CHECKING

from trezor import wire

from apps.common import paths
from apps.common.keychain import get_keychain

from . import CURVE, networks, definitions

if TYPE_CHECKING:
    from typing import Awaitable, Callable, Iterable, TypeVar

    from trezor.messages import (
        EthereumGetAddress,
        EthereumGetPublicKey,
        EthereumSignMessage,
        EthereumSignTx,
        EthereumSignTxEIP1559,
        EthereumSignTypedData,
    )

    from apps.common.keychain import MsgIn as MsgInGeneric, MsgOut, Handler

    EthereumMessages = (
        EthereumGetAddress
        | EthereumGetPublicKey
        | EthereumSignTx
        | EthereumSignMessage
        | EthereumSignTypedData
    )
    MsgIn = TypeVar("MsgIn", bound=EthereumMessages)

    EthereumSignTxAny = EthereumSignTx | EthereumSignTxEIP1559
    MsgInChainId = TypeVar("MsgInChainId", bound=EthereumSignTxAny)

    # TODO: check the types of messages
    HandlerWithKeychainAndDefinitions = Callable[[wire.Context, MsgInGeneric, definitions.EthereumDefinitions], Awaitable[MsgOut]]


# We believe Ethereum should use 44'/60'/a' for everything, because it is
# account-based, rather than UTXO-based. Unfortunately, lot of Ethereum
# tools (MEW, Metamask) do not use such scheme and set a = 0 and then
# iterate the address index i. For compatibility, we allow this scheme as well.

PATTERNS_ADDRESS = (paths.PATTERN_BIP44, paths.PATTERN_SEP5)


def _schemas_from_address_n(
    patterns: Iterable[str], address_n: paths.Bip32Path, network_info: networks.NetworkInfo | None
) -> Iterable[paths.PathSchema]:
    if len(address_n) < 2:
        return ()

    slip44_hardened = address_n[1]

    # check with network from definitions and if that is None then with built-in ones
    if slip44_hardened != network_info.slip44 | paths.HARDENED or slip44_hardened not in networks.all_slip44_ids_hardened():
        return ()

    if not slip44_hardened & paths.HARDENED:
        return ()

    slip44_id = slip44_hardened - paths.HARDENED
    schemas = [paths.PathSchema.parse(pattern, slip44_id) for pattern in patterns]
    return [s.copy() for s in schemas]


def with_keychain_from_path(
    *patterns: str,
) -> Callable[[HandlerWithKeychainAndDefinitions[MsgIn, MsgOut]], Handler[MsgIn, MsgOut]]:
    def decorator(func: HandlerWithKeychainAndDefinitions[MsgIn, MsgOut]) -> Handler[MsgIn, MsgOut]:
        async def wrapper(ctx: wire.Context, msg: MsgIn) -> MsgOut:
            defs = definitions.EthereumDefinitions(definitions.get_encoded_definitions_from_msg(msg))
            schemas = _schemas_from_address_n(patterns, msg.address_n, defs.network)
            keychain = await get_keychain(ctx, CURVE, schemas)
            with keychain:
                return await func(ctx, msg, keychain, defs)

        return wrapper

    return decorator


def _schemas_from_chain_id(network_info: networks.NetworkInfo | None) -> Iterable[paths.PathSchema]:
    slip44_id: tuple[int, ...]
    if network_info is None:
        # allow Ethereum or testnet paths for unknown networks
        slip44_id = (60, 1)
    elif network_info.slip44 not in (60, 1):
        # allow cross-signing with Ethereum unless it's testnet
        slip44_id = (network_info.slip44, 60)
    else:
        slip44_id = (network_info.slip44,)

    schemas = [
        paths.PathSchema.parse(pattern, slip44_id) for pattern in PATTERNS_ADDRESS
    ]
    return [s.copy() for s in schemas]


def with_keychain_from_chain_id(
    func: HandlerWithKeychainAndDefinitions[MsgInChainId, MsgOut]
) -> Handler[MsgInChainId, MsgOut]:
    # this is only for SignTx, and only PATTERN_ADDRESS is allowed
    async def wrapper(ctx: wire.Context, msg: MsgInChainId) -> MsgOut:
        defs = definitions.EthereumDefinitions(definitions.get_encoded_definitions_from_msg(msg))
        schemas = _schemas_from_chain_id(defs.network)
        keychain = await get_keychain(ctx, CURVE, schemas)
        with keychain:
            return await func(ctx, msg, keychain, defs)

    return wrapper
