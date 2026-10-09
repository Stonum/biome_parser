use biome_rowan::SyntaxKind;
use std::marker::PhantomData;

/// Number of `u128` words in a [TokenSet]: up to `TOKEN_SET_WORDS * 128`
/// token kinds.
const TOKEN_SET_WORDS: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenSet<K: SyntaxKind>([u128; TOKEN_SET_WORDS], PhantomData<K>);

impl<K: SyntaxKind> TokenSet<K> {
    pub const EMPTY: TokenSet<K> = TokenSet([0; TOKEN_SET_WORDS], PhantomData);

    pub fn singleton(kind: K) -> Self {
        unsafe { TokenSet::from_raw(kind.to_raw().0) }
    }

    pub const fn union(self, other: TokenSet<K>) -> Self {
        let mut bits = [0; TOKEN_SET_WORDS];
        let mut index = 0;
        while index < TOKEN_SET_WORDS {
            bits[index] = self.0[index] | other.0[index];
            index += 1;
        }
        TokenSet(bits, PhantomData)
    }

    pub fn contains(&self, kind: K) -> bool {
        let num = kind.to_raw().0 as usize;
        let index = num / u128::BITS as usize;
        index < TOKEN_SET_WORDS && self.0[index] & (1u128 << (num % u128::BITS as usize)) != 0
    }

    /// Constructs a token set for a single kind from a kind's raw `u16` representation.
    ///
    /// # Safety
    ///
    /// This method is marked unsafe to discourage its usage over using `TokenSet::singleton`.
    /// It exists to support the `token_set` macro in a `const` context.
    #[doc(hidden)]
    pub const unsafe fn from_raw(kind: u16) -> Self {
        TokenSet(mask(kind), PhantomData)
    }
}

const fn mask(kind: u16) -> [u128; TOKEN_SET_WORDS] {
    let num = kind as usize;
    let mut bits = [0; TOKEN_SET_WORDS];
    bits[num / u128::BITS as usize] = 1u128 << (num % u128::BITS as usize);
    bits
}

/// Utility macro for making a new token set
#[macro_export]
macro_rules! token_set {
    ($($t:expr),*) => {{
            use $crate::TokenSet;
            TokenSet::EMPTY$(.union(unsafe { TokenSet::from_raw($t as u16) }))*
        }};
    ($($t:expr),* ,) => { token_set!($($t),*) };
}
