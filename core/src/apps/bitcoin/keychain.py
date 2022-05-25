from typing import TYPE_CHECKING

from trezor import wire
from trezor.messages import AuthorizeCoinJoin, SignTx

from apps.common import coininfo
from apps.common.keychain import get_keychain

from . import authorization
from .paths import get_schemas_for_coin

if TYPE_CHECKING:
    from typing import Awaitable, Callable, TypeVar

    from trezor.protobuf import MessageType

    from trezor.messages import (
        GetAddress,
        GetOwnershipId,
        GetOwnershipProof,
        GetPublicKey,
        SignMessage,
        VerifyMessage,
    )

    from apps.common.keychain import Keychain, MsgOut, Handler

    BitcoinMessage = (
        AuthorizeCoinJoin
        | GetAddress
        | GetOwnershipId
        | GetOwnershipProof
        | GetPublicKey
        | SignMessage
        | SignTx
        | VerifyMessage
    )

    MsgIn = TypeVar("MsgIn", bound=BitcoinMessage)
    HandlerWithCoinInfo = Callable[..., Awaitable[MsgOut]]


def get_coin_by_name(coin_name: str | None) -> coininfo.CoinInfo:
    if coin_name is None:
        coin_name = "Bitcoin"

    try:
        return coininfo.by_name(coin_name)
    except ValueError:
        raise wire.DataError("Unsupported coin type")


async def get_keychain_for_coin(
    ctx: wire.Context,
    coin_name: str | None,
    allow_slip25_internal: bool = False,
) -> tuple[Keychain, coininfo.CoinInfo]:
    coin = get_coin_by_name(coin_name)
    schemas = get_schemas_for_coin(coin, allow_slip25_internal)
    slip21_namespaces = [[b"SLIP-0019"], [b"SLIP-0024"]]
    keychain = await get_keychain(ctx, coin.curve_name, schemas, slip21_namespaces)
    return keychain, coin


def with_keychain(func: HandlerWithCoinInfo[MsgOut]) -> Handler[MsgIn, MsgOut]:
    async def wrapper(
        ctx: wire.Context,
        msg: MsgIn,
        auth_msg: MessageType | None = None,
    ) -> MsgOut:
        # Allow access to the SLIP25 internal chain (CoinJoin) for SignTx and preauthorized CoinJoin operations.
        allow_slip25_internal = SignTx.is_type_of(msg) or (
            auth_msg is not None and AuthorizeCoinJoin.is_type_of(auth_msg)
        )
        keychain, coin = await get_keychain_for_coin(
            ctx, msg.coin_name, allow_slip25_internal
        )
        if auth_msg:
            auth_obj = authorization.from_cached_message(auth_msg)
            return await func(ctx, msg, keychain, coin, auth_obj)
        else:
            with keychain:
                return await func(ctx, msg, keychain, coin)

    return wrapper
