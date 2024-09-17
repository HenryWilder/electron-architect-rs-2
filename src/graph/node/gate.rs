pub enum Gate {
    Always,
    Never,

    Not,

    Xor,

    And,
    Nand,
    Or,
    Nor,
}

impl Gate {
    pub const fn max_inputs(&self) -> usize {
        match self {
            | Self::Always
            | Self::Never
                => 0,

            | Self::Not
                => 1,

            | Self::Xor
                => 2,

            | Self::And
            | Self::Nand
            | Self::Or
            | Self::Nor
                => usize::MAX,
        }
    }

    pub fn evaluate(&self, mut inputs: impl Iterator<Item = bool>) -> bool {
        match self {
            Self::Always => true,
            Self::Never  => false,

            Self::Not => !inputs.next().unwrap_or_default(),

            Self::Xor => inputs.next().unwrap_or_default() ^ inputs.next().unwrap_or_default(),

            Self::And  =>  inputs.all(|x| x),
            Self::Nand => !inputs.all(|x| x),
            Self::Or   =>  inputs.any(|x| x),
            Self::Nor  => !inputs.any(|x| x),
        }
    }
}
