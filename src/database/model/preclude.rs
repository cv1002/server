macro_rules! declare_models {
    ($($mod:ident => $model:ident, $entity:ident);* $(;)?) => {
        $(
            pub use super::$mod::Model as $model;
            pub use super::$mod::Entity as $entity;
        ) *
    };
}

declare_models! {}
