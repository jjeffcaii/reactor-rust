// use std::marker::PhantomData;

// pub trait Flux<T>: Pub<Item = T> {
//     fn map<B, P, M>(self, transform: M) -> FluxMap<P, M, T, B>
//     where
//         P: Pub<Item = T>,
//         M: Fn(&T) -> B;
// }

// pub trait Pub {
//     type Item;

//     fn sub(&self, func: impl Fn(&Self::Item));
// }

// pub struct FluxMap<P, M, T1, T2> {
//     source: P,
//     mapper: M,
//     _t1: PhantomData<T1>,
//     _t2: PhantomData<T2>,
// }

// impl<P, M, T1, T2> FluxMap<P, M, T1, T2>
// where
//     P: Pub<Item = T1>,
//     M: Fn(&T1) -> T2,
// {
//     pub fn new(source: P, mapper: M) -> FluxMap<P, M, T1, T2> {
//         FluxMap {
//             source,
//             mapper,
//             _t1: PhantomData,
//             _t2: PhantomData,
//         }
//     }
// }

// impl<P, M, T1, T2> Pub for FluxMap<P, M, T1, T2>
// where
//     P: Pub<Item = T1>,
//     M: Fn(&T1) -> T2,
// {
//     type Item = T2;

//     fn sub(&self, func: impl Fn(&Self::Item)) {
//         self.source.sub(|a| {
//             let b = (self.mapper)(a);
//             func(&b);
//         });
//     }
// }

// pub struct FluxVec<T> {
//     values: Vec<T>,
// }

// impl<T> FluxVec<T> {
//     pub fn new(values: Vec<T>) -> FluxVec<T> {
//         FluxVec { values }
//     }
// }

// impl<T> Flux<T> for FluxVec<T> {
//     fn map<B, P, M>(self, transform: M) -> FluxMap<P, M, T, B>
//     where
//         P: Pub<Item = T>,
//         M: Fn(&T) -> B,
//     {
//         FluxMap::new(self, transform)
//     }
// }

// impl<T> Pub for FluxVec<T> {
//     type Item = T;

//     fn sub(&self, func: impl Fn(&Self::Item)) {
//         for it in self.values.iter() {
//             func(it);
//         }
//     }
// }
