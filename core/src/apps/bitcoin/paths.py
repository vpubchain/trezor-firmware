import gc
from micropython import const
from typing import TYPE_CHECKING

from trezor.enums import InputScriptType

from apps.common.paths import PATTERN_BIP44, UNHARDEN_MASK, PathSchema

from .common import BITCOIN_NAMES

if TYPE_CHECKING:
    from typing import Iterable
    from typing_extensions import Protocol

    from apps.common.coininfo import CoinInfo
    from apps.common.paths import Bip32Path

    class MsgWithAddressScriptType(Protocol):
        address_n: Bip32Path
        script_type: InputScriptType


# BIP-45 for multisig: https://github.com/bitcoin/bips/blob/master/bip-0045.mediawiki
PATTERN_BIP45 = "m/45'/[0-100]/change/address_index"

# BIP-48 for multisig: https://github.com/bitcoin/bips/blob/master/bip-0048.mediawiki
# The raw script type is not part of the BIP (and Electrum, as a notable implementation,
# does not use it), it is included here for completeness.
PATTERN_BIP48_RAW = "m/48'/coin_type'/account'/0'/change/address_index"
PATTERN_BIP48_P2SHSEGWIT = "m/48'/coin_type'/account'/1'/change/address_index"
PATTERN_BIP48_SEGWIT = "m/48'/coin_type'/account'/2'/change/address_index"

# BIP-49 for segwit-in-P2SH: https://github.com/bitcoin/bips/blob/master/bip-0049.mediawiki
PATTERN_BIP49 = "m/49'/coin_type'/account'/change/address_index"
# BIP-84 for segwit: https://github.com/bitcoin/bips/blob/master/bip-0084.mediawiki
PATTERN_BIP84 = "m/84'/coin_type'/account'/change/address_index"
# BIP-86 for taproot: https://github.com/bitcoin/bips/blob/master/bip-0086.mediawiki
PATTERN_BIP86 = "m/86'/coin_type'/account'/change/address_index"
# SLIP-25 for CoinJoin: https://github.com/satoshilabs/slips/blob/master/slip-0025.md
# Only account=0 and script_type=1 are supported for now.
PATTERN_SLIP25 = "m/10025'/coin_type'/0'/1'/change/address_index"
PATTERN_SLIP25_EXTERNAL = "m/10025'/coin_type'/0'/1'/0/address_index"

# compatibility patterns, will be removed in the future
PATTERN_GREENADDRESS_A = "m/[1,4]/address_index"
PATTERN_GREENADDRESS_B = "m/3'/[1-100]'/[1,4]/address_index"
PATTERN_GREENADDRESS_SIGN_A = "m/1195487518"
PATTERN_GREENADDRESS_SIGN_B = "m/1195487518/6/address_index"

PATTERN_CASA = "m/49/coin_type/account/change/address_index"

PATTERN_UNCHAINED_HARDENED = (
    "m/45'/coin_type'/account'/[0-1000000]/change/address_index"
)
PATTERN_UNCHAINED_UNHARDENED = (
    "m/45'/coin_type/account/[0-1000000]/change/address_index"
)
PATTERN_UNCHAINED_DEPRECATED = "m/45'/coin_type'/account'/[0-1000000]/address_index"

# SLIP-44 coin type for Bitcoin
SLIP44_BITCOIN = const(0)

# SLIP-44 coin type for all Testnet coins
SLIP44_TESTNET = const(1)


def validate_path_against_script_type(
    coin: CoinInfo,
    msg: MsgWithAddressScriptType | None = None,
    address_n: Bip32Path | None = None,
    script_type: InputScriptType | None = None,
    multisig: bool = False,
) -> bool:
    patterns = []

    if msg is not None:
        assert address_n is None and script_type is None
        address_n = msg.address_n
        script_type = msg.script_type or InputScriptType.SPENDADDRESS
        multisig = bool(getattr(msg, "multisig", False))

    else:
        assert address_n is not None and script_type is not None

    if script_type == InputScriptType.SPENDADDRESS and not multisig:
        patterns.append(PATTERN_BIP44)
        if coin.slip44 == SLIP44_BITCOIN:
            patterns.append(PATTERN_GREENADDRESS_A)
            patterns.append(PATTERN_GREENADDRESS_B)

    elif (
        script_type in (InputScriptType.SPENDADDRESS, InputScriptType.SPENDMULTISIG)
        and multisig
    ):
        patterns.append(PATTERN_BIP48_RAW)
        if coin.slip44 == SLIP44_BITCOIN or (
            coin.fork_id is not None and coin.slip44 != SLIP44_TESTNET
        ):
            patterns.append(PATTERN_BIP45)
        if coin.slip44 == SLIP44_BITCOIN:
            patterns.append(PATTERN_GREENADDRESS_A)
            patterns.append(PATTERN_GREENADDRESS_B)
        if coin.coin_name in BITCOIN_NAMES:
            patterns.append(PATTERN_UNCHAINED_HARDENED)
            patterns.append(PATTERN_UNCHAINED_UNHARDENED)
            patterns.append(PATTERN_UNCHAINED_DEPRECATED)

    elif coin.segwit and script_type == InputScriptType.SPENDP2SHWITNESS:
        patterns.append(PATTERN_BIP49)
        if multisig:
            patterns.append(PATTERN_BIP48_P2SHSEGWIT)
        if coin.slip44 == SLIP44_BITCOIN:
            patterns.append(PATTERN_GREENADDRESS_A)
            patterns.append(PATTERN_GREENADDRESS_B)
        if coin.coin_name in BITCOIN_NAMES:
            patterns.append(PATTERN_CASA)

    elif coin.segwit and script_type == InputScriptType.SPENDWITNESS:
        patterns.append(PATTERN_BIP84)
        if multisig:
            patterns.append(PATTERN_BIP48_SEGWIT)
        if coin.slip44 == SLIP44_BITCOIN:
            patterns.append(PATTERN_GREENADDRESS_A)
            patterns.append(PATTERN_GREENADDRESS_B)

    elif coin.taproot and script_type == InputScriptType.SPENDTAPROOT:
        patterns.append(PATTERN_BIP86)
        patterns.append(PATTERN_SLIP25)

    return any(
        PathSchema.parse(pattern, coin.slip44).match(address_n) for pattern in patterns
    )


def get_schemas_for_coin(
    coin: CoinInfo, allow_slip25_internal: bool = False
) -> Iterable[PathSchema]:
    # basic patterns
    patterns = [
        PATTERN_BIP44,
        PATTERN_BIP48_RAW,
    ]

    # patterns without coin_type field must be treated as if coin_type == 0
    if coin.slip44 == SLIP44_BITCOIN or (
        coin.fork_id is not None and coin.slip44 != SLIP44_TESTNET
    ):
        patterns.append(PATTERN_BIP45)

    if coin.slip44 == SLIP44_BITCOIN:
        patterns.extend(
            (
                PATTERN_GREENADDRESS_A,
                PATTERN_GREENADDRESS_B,
                PATTERN_GREENADDRESS_SIGN_A,
                PATTERN_GREENADDRESS_SIGN_B,
            )
        )

    # compatibility patterns
    if coin.coin_name in BITCOIN_NAMES:
        patterns.extend(
            (
                PATTERN_CASA,
                PATTERN_UNCHAINED_HARDENED,
                PATTERN_UNCHAINED_UNHARDENED,
                PATTERN_UNCHAINED_DEPRECATED,
            )
        )

    # segwit patterns
    if coin.segwit:
        patterns.extend(
            (
                PATTERN_BIP49,
                PATTERN_BIP84,
                PATTERN_BIP48_P2SHSEGWIT,
                PATTERN_BIP48_SEGWIT,
            )
        )

    # taproot patterns
    if coin.taproot:
        patterns.append(PATTERN_BIP86)
        if allow_slip25_internal:
            patterns.append(PATTERN_SLIP25)
        else:
            patterns.append(PATTERN_SLIP25_EXTERNAL)

    schemas = [PathSchema.parse(pattern, coin.slip44) for pattern in patterns]

    # Some wallets such as Electron-Cash (BCH) store coins on Bitcoin paths.
    # We can allow spending these coins from Bitcoin paths if the coin has
    # implemented strong replay protection via SIGHASH_FORKID. However, we
    # cannot allow spending any testnet coins from Bitcoin paths, because
    # otherwise an attacker could trick the user into spending BCH on a Bitcoin
    # path by signing a seemingly harmless BCH Testnet transaction.
    if coin.fork_id is not None and coin.slip44 != SLIP44_TESTNET:
        schemas.extend(
            PathSchema.parse(pattern, SLIP44_BITCOIN) for pattern in patterns
        )

    gc.collect()
    return [schema.copy() for schema in schemas]


def address_n_to_name(address_n: list[int], coin: CoinInfo) -> str | None:
    patterns: list[tuple[str, str]] = [(PATTERN_BIP44, "Legacy")]
    if coin.segwit:
        patterns.append((PATTERN_BIP49, "Legacy SegWit"))
        patterns.append((PATTERN_BIP84, "SegWit"))

    if coin.taproot:
        patterns.append((PATTERN_BIP86, "Taproot"))
        patterns.append((PATTERN_SLIP25, "CoinJoin"))

    for pattern, account_type in patterns:
        if PathSchema.parse(pattern, coin.slip44).match(address_n):
            account_number = (address_n[2] & UNHARDEN_MASK) + 1
            if len(patterns) == 1:
                return f"account #{account_number}"
            else:
                return f"{account_type} account #{account_number}"

    return None
