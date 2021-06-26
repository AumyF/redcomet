//! 命令語の解釈とその処理の実行

#[derive(Debug, Clone)]
pub enum Operations {
    NoOperation,
    Load2,
    Store,
    LoadAddress,
    Load1,
    AddArithmetic1(TwoRegisters),
    SubtractArithmetic,
    AddLogical,
    SubtratLogical,
}

#[derive(Debug, thiserror::Error)]
pub enum NewError {
    #[error("Operation not defined for word {0:X}")]
    OperationNotDefined(u16),
}

impl Operations {
    pub fn new(word: u16) -> Result<Operations, NewError> {
        use Operations::*;
        Ok(match word & 0xff00 {
            0 => NoOperation, // NOP
            0x2400 => AddArithmetic1(TwoRegisters {
                r1: (word & 0x00f0) >> 1,
                r2: (word & 0x000f),
            }),
            e => Err(NewError::OperationNotDefined(e))?,
        })
    }
}

/// GRのインデックスのペア。R1 <- f (R1, R2) みたいな演算で使う。
/// `new` で範囲内 (R <= 7) であることを保証しているので
/// これを使うときは範囲外アクセスを気にして `Result` を使う必要はない。
#[derive(Debug, Clone, Copy)]
pub struct TwoRegisters {
    r1: u16,
    r2: u16,
}

impl TwoRegisters {
    fn new(r1: u16, r2: u16) -> Result<TwoRegisters, (u16, u16)> {
        if r1 > 7 || r2 > 7 {
            Err((r1, r2))
        } else {
            Ok(TwoRegisters { r1, r2 })
        }
    }
    pub fn get_pair(&self) -> (&u16, &u16) {
        let Self { r1, r2 } = self;
        (r1, r2)
    }
}

mod test {
    use std::{error, io};

    use crate::core::machine::Machine;

    // TODO テスト書く
    // #[test]
    // fn add_arithmetic_1() -> Result<(), Box<dyn error::Error>> {
    //     let vec = vec![0x00, 0x24, 0x12];
    //     let mut bytes = io::Cursor::new(vec);
    //     let machine = Machine::init(&mut bytes);
    //     // assert_eq!()

    //     Ok(())
    // }
}
