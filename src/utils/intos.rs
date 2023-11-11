use std::{
    rc::Rc,
    sync::{Arc, Mutex, RwLock},
};

/// Some extension function...
pub trait IntoExtensions: Sized {
    /// IntoSome extension function
    /// # Examples
    ///
    /// ```rust
    /// use utils::intos::IntoExtensions;
    ///
    /// let ret = 1.into_some();
    /// assert_eq!(ret, Some(1));
    /// ```
    fn into_some(self) -> Option<Self>;
    /// IntoOk extension function
    /// # Examples
    ///
    /// ```rust
    /// use utils::intos::IntoExtensions;
    ///
    /// let ret = 1.into_ok::<()>();
    /// assert_eq!(ret, Ok(1));
    /// ```
    fn into_ok<E>(self) -> Result<Self, E>;
    /// IntoErr extension function
    /// # Examples
    ///
    /// ```rust
    /// use utils::intos::IntoExtensions;
    ///
    /// let ret = 1.into_err::<()>();
    /// assert_eq!(ret, Err(1));
    /// ```
    fn into_err<T>(self) -> Result<T, Self>;
    /// IntoArc extension function
    /// # Examples
    ///
    /// ```rust
    /// use utils::intos::IntoExtensions;
    /// use std::{any::{Any, TypeId}, sync::Arc};
    ///
    /// let ret = 1.into_arc();
    /// assert_eq!(ret.type_id(), TypeId::of::<Arc<i32>>());
    /// ```
    fn into_arc(self) -> Arc<Self>;
    /// IntoBox extension function
    /// # Examples
    ///
    /// ```rust
    /// use utils::intos::IntoExtensions;
    /// use std::any::{Any, TypeId};
    ///
    /// let ret = 1.into_box();
    /// assert_eq!(ret.type_id(), TypeId::of::<Box<i32>>());
    /// ```
    fn into_box(self) -> Box<Self>;
    /// IntoRc extension function
    /// # Examples
    ///
    /// ```rust
    /// use utils::intos::IntoExtensions;
    /// use std::{any::{Any, TypeId}, rc::Rc};
    ///
    /// let ret = 1_i32.into_rc();
    /// assert_eq!(ret.type_id(), TypeId::of::<Rc<i32>>());
    /// ```
    fn into_rc(self) -> Rc<Self>;
    /// IntoMutex extension function
    /// # Examples
    ///
    /// ```rust
    /// use utils::intos::IntoExtensions;
    /// use std::{any::{Any, TypeId}, sync::Mutex};
    ///
    /// let ret = 1_i32.into_mutex();
    /// assert_eq!(ret.type_id(), TypeId::of::<Mutex<i32>>());
    /// ```
    fn into_mutex(self) -> Mutex<Self>;
    /// IntoRwLock extension function
    /// # Examples
    ///
    /// ```rust
    /// use utils::intos::IntoExtensions;
    /// use std::{any::{Any, TypeId}, sync::RwLock};
    ///
    /// let ret = 1_i32.into_rwlock();
    /// assert_eq!(ret.type_id(), TypeId::of::<RwLock<i32>>());
    /// ```
    fn into_rwlock(self) -> RwLock<Self>;
}

impl<Ty: Sized> IntoExtensions for Ty {
    #[inline(always)]
    fn into_some(self) -> Option<Self> {
        Some(self)
    }

    #[inline(always)]
    fn into_ok<E>(self) -> Result<Self, E> {
        Ok(self)
    }

    #[inline(always)]
    fn into_err<T>(self) -> Result<T, Self> {
        Err(self)
    }

    #[inline(always)]
    fn into_arc(self) -> Arc<Self> {
        Arc::new(self)
    }

    #[inline(always)]
    fn into_box(self) -> Box<Self> {
        Box::new(self)
    }

    #[inline(always)]
    fn into_rc(self) -> Rc<Self> {
        Rc::new(self)
    }

    #[inline(always)]
    fn into_mutex(self) -> Mutex<Self> {
        Mutex::new(self)
    }

    #[inline(always)]
    fn into_rwlock(self) -> RwLock<Self> {
        RwLock::new(self)
    }
}
