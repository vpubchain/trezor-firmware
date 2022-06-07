import pytest

from ..src.bin_size.row_handler_rust import RustRow
from .common import mock_data_row

RR = RustRow()


@pytest.mark.parametrize(
    "symbol,module,func",
    [
        (
            "trezor_lib::protobuf::decode::Decoder::decode_field::hab425281b2042fd5",
            "embed/rust/src/protobuf/decode.rs",
            "Decoder::decode_field()",
        ),
        (
            "trezor_lib::protobuf::obj::msg_obj_attr::h59281a534905240d",
            "embed/rust/src/protobuf/obj.rs",
            "msg_obj_attr()",
        ),
        (
            "trezor_lib::protobuf::nonexisting::obj::msg_obj_attr::h59281a534905240d",
            "--invalid_file--embed/rust/src/protobuf/nonexisting/obj.rs",
            "msg_obj_attr()",
        ),
        (
            "trezor_lib::micropython::list::_$LT$impl$u20$trezor_lib..micropython..ffi.._mp_obj_list_t$GT$::alloc::h988fbb6155b3d81e",
            "embed/rust/src/micropython/list.rs",
            "alloc()",
        ),
        (
            "trezor_lib::util::try_or_raise::_$u7b$$u7b$closure$u7d$$u7d$::h059f3c8d3819af81",
            "embed/rust/src/util.rs",
            "try_or_raise()",
        ),
        (
            "trezor_lib::protobuf::obj::MsgDefObj::obj_type::TYPE::hf415c733c642ed60",
            "embed/rust/src/protobuf/obj.rs",
            "MsgDefObj::obj_type()",
        ),
        (
            "_$LT$trezor_lib..protobuf..encode..BufferStream$u20$as$u20$trezor_lib..protobuf..encode..OutputStream$GT$::write",
            "",
            "",
        ),
    ],
)
def test_get_module_and_function(symbol: str, module: str, func: str):
    assert RR._get_module_and_function(symbol) == (module, func)


def test_add_basic_info_row_handlers():
    new_row = RR.add_basic_info(
        mock_data_row(
            symbol_name="trezor_lib::protobuf::decode::Decoder::decode_field::hab425281b2042fd5"
        )
    )
    assert new_row.language == "Rust"
    assert new_row.module_name == "embed/rust/src/protobuf/decode.rs"
    assert new_row.func_name == "Decoder::decode_field()"


@pytest.mark.parametrize(
    "module,func,definition",
    [
        (
            "embed/rust/src/protobuf/encode.rs",
            "Encoder::encode_field()",
            "embed/rust/src/protobuf/encode.rs:95",
        ),
        (
            "embed/rust/src/protobuf/obj.rs",
            "msg_obj_attr()",
            "embed/rust/src/protobuf/obj.rs:132",
        ),
        (
            "embed/rust/src/micropython/runtime.rs",
            "trampoline()",
            "embed/rust/src/micropython/runtime.rs:68",
        ),
        (
            "embed/rust/src/protobuf/obj.rs",
            "unexisting_msg_obj_attr()",
            "",
        ),
        (
            "embed/rust/src/unexisting_protobuf/obj.rs",
            "msg_obj_attr()",
            "",
        ),
    ],
)
def test_get_definition(module: str, func: str, definition: str):
    assert (
        RR._get_definition(mock_data_row(module_name=module, func_name=func))
        == definition
    )
