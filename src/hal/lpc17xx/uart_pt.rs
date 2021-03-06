// Zinc, the bare metal stack for rust.
// Copyright 2014 Vladimir "farcaller" Pouzanov <farcaller@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::gc::Gc;
use syntax::ext::base::ExtCtxt;

use builder::{Builder, TokenString};
use node;

pub fn build_uart(builder: &mut Builder, cx: &mut ExtCtxt,
    node: &Gc<node::Node>) {
  if !node.expect_no_attributes(cx) { return }

  for (path, sub) in node.subnodes.iter() {
    let uart_peripheral_str = format!("UART{}",
        match from_str::<uint>(path.as_slice()).unwrap() {
          0|2|3 => path,
          other => {
            cx.parse_sess().span_diagnostic.span_err(sub.path_span,
                format!("unknown UART `{}`, allowed values: 0, 2, 3",
                    other).as_slice());
            continue;
          }
        });
    let uart_peripheral = TokenString(uart_peripheral_str);

    if sub.name.is_none() {
      cx.parse_sess().span_diagnostic.span_err(sub.name_span,
          "UART node must have a name");
      continue;
    }

    if !sub.expect_attributes(cx, [
        ("baud_rate", node::IntAttribute),
        ("mode", node::StrAttribute),
        ("tx", node::RefAttribute),
        ("rx", node::RefAttribute)]) {
      continue;
    }

    let baud_rate: u32 = sub.get_int_attr("baud_rate").unwrap() as u32;
    let mode = sub.get_string_attr("mode").unwrap();
    let tx_node_name = sub.get_ref_attr("tx").unwrap();
    let rx_node_name = sub.get_ref_attr("rx").unwrap();

    let word_len = mode.as_slice().char_at(0).to_digit(10).unwrap() as u8;
    let parity = TokenString(
        match mode.as_slice().char_at(1) {
          'N' => "Disabled",
          'O' => "Odd",
          'E' => "Even",
          '1' => "Forced1",
          '0' => "Forced0",
          _ => fail!(),
        }.to_str());
    let stop_bits = mode.as_slice().char_at(2).to_digit(10).unwrap() as u8;
    build_uart_gpio(builder, from_str(path.as_slice()).unwrap(),
        tx_node_name.as_slice(), true);
    build_uart_gpio(builder, from_str(path.as_slice()).unwrap(),
        rx_node_name.as_slice(), false);

    sub.type_name.set(Some("zinc::hal::lpc17xx::uart::UART"));
    let uart_name = TokenString(sub.name.clone().unwrap());

    let st = quote_stmt!(&*cx,
        let $uart_name = zinc::hal::lpc17xx::uart::UART::new(
            zinc::hal::lpc17xx::uart::$uart_peripheral,
            $baud_rate,
            $word_len,
            zinc::hal::uart::$parity,
            $stop_bits)
    );
    builder.add_main_statement(st);
  }
}

pub fn build_uart_gpio(builder: &Builder, uart_idx: uint, name: &str,
    istx: bool) {
  let node = builder.pt.get_by_name(name).unwrap();
  let direction = (if istx {"out"} else {"in"}).to_str();
  let function = format!("{}{}", if istx {"txd"} else {"rxd"}, uart_idx);
  node.attributes.borrow_mut().insert("direction".to_str(),
        node::Attribute::new_nosp(node::StrValue(direction)));
  node.attributes.borrow_mut().insert("function".to_str(),
        node::Attribute::new_nosp(node::StrValue(function)));
}

#[cfg(test)]
mod test {
  use builder::Builder;
  use test_helpers::{assert_equal_source, with_parsed};

  #[test]
  fn builds_uart() {
    with_parsed("
      mcu {
        clock {
          system_frequency = 100_000_000;
        }
      }
      uart {
        uart@0 {
          baud_rate = 9600;
          mode = \"8N1\";
          tx = &uart_tx;
          rx = &uart_rx;
        }
      }
      gpio {
        uart_tx@0;
        uart_rx@1;
      }
      ", |cx, failed, pt| {
      let mut builder = Builder::new(pt);
      super::build_uart(&mut builder, cx, pt.get_by_path("uart").unwrap());
      assert!(unsafe{*failed} == false);
      assert!(builder.main_stmts.len() == 1);

      assert_equal_source(builder.main_stmts.get(0),
          "let uart = zinc::hal::lpc17xx::uart::UART::new(
               zinc::hal::lpc17xx::uart::UART0,
               9600u32,
               8u8,
               zinc::hal::uart::Disabled,
               1u8);");

      let tx_node = pt.get_by_name("uart_tx").unwrap();
      assert!(tx_node.get_string_attr("direction").unwrap() == "out".to_str());
      assert!(tx_node.get_string_attr("function").unwrap() == "txd0".to_str());

      let rx_node = pt.get_by_name("uart_rx").unwrap();
      assert!(rx_node.get_string_attr("direction").unwrap() == "in".to_str());
      assert!(rx_node.get_string_attr("function").unwrap() == "rxd0".to_str());
    });
  }
}
