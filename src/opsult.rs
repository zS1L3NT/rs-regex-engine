pub enum Opsult<T, E> {
    Some(T),
    Err(E),
    None,
}

impl<T, E> Opsult<T, E> {
    pub fn unwrap_some(self) -> T {
        match self {
            Opsult::Some(t) => t,
            Opsult::Err(_) => panic!("unwrap_some called on Err(E)"),
            Opsult::None => panic!("unwrap_some called on None"),
        }
    }

    pub fn unwrap_err(self) -> E {
        match self {
            Opsult::Some(_) => panic!("unwrap_err called on Some(T)"),
            Opsult::Err(e) => e,
            Opsult::None => panic!("unwrap_err called on None"),
        }
    }

    pub fn unwrap_none(self) -> () {
        match self {
            Opsult::Some(_) => panic!("unwrap_none called on Some(T)"),
            Opsult::Err(_) => panic!("unwrap_none called on Err(E)"),
            Opsult::None => (),
        }
    }

    pub fn is_some(self) -> bool {
        match self {
            Opsult::Some(_) => true,
            Opsult::Err(_) => false,
            Opsult::None => false,
        }
    }

    pub fn is_err(self) -> bool {
        match self {
            Opsult::Some(_) => false,
            Opsult::Err(_) => true,
            Opsult::None => false,
        }
    }

    pub fn is_none(self) -> bool {
        match self {
            Opsult::Some(_) => false,
            Opsult::Err(_) => false,
            Opsult::None => true,
        }
    }
}
