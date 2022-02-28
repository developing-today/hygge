use std::collections::hash_map::DefaultHasher;

fn main() {
    // 0.3.0
    //     let v = KdlValueBuilder;
    //     let mut kdl = KdlNodeBuilder::new("hygge");
    //     let mut childs = KdlNodeBuilder::new("child")
    //         .bool(true)
    //         .prop("uuid".to_string(), v.str("123".to_string()).to_owned());
    //     kdl = kdl
    //         .nil()
    //         .nil()
    //         .y()
    //         .n()
    //         .child(childs.clone())
    //         .child(childs.clone().name("child3"))
    //         .str("hello")
    //         .child(KdlNodeBuilder::new("child2"));
    //     println!("{:#?}", kdl.build());

    //     let mut kdl = KdlNodeBuilder::new("hygge")
    //         .nil()
    //         .nil()
    //         .y()
    //         .n()
    //         .child(childs.clone())
    //         .child(childs.clone().name("child3"))
    //         .str("hello")
    //         .child(KdlNodeBuilder::new("child2"))
    //         .child(KdlNodeBuilder::new("child4"))
    //         .prop("asd".to_string(), v.flt(1.into()));
    //     let two = &kdl.clone().child(kdl.name("hygge2")).prop("asd", v.str(2));
    //     println!("{:#?}", two.build());

    // 0.4.0
    let v = KdlValueBuilder;
    println!(
        "{:#?}",
        KdlNodeBuilder::new(&mut "hygge".to_string())
            .nil()
            .nil()
            .y()
            .n()
            .str(&mut "hello".to_string())
            .prop(&mut "asd".to_string(), v.flt(1.into()))
            .build()
    );

    // 0.5.0 ->
    let mut kdl = KdlNodeBuilder::new(&mut "hygge".to_string());
    let mut childs = KdlNodeBuilder::new(&mut "child".to_string())
        .bool(true)
        .prop(
            &mut "uuid".to_string(),
            v.str(&"123".to_string()).to_owned(),
        );
    println!(
        "{:#?}",
        kdl.nil()
            .nil()
            .y()
            .n()
            // .child(childs.clone())
            // .child(childs.clone())
            .str(&mut "hello".to_string())
            .child(KdlNodeBuilder::new(&mut "child2".to_string()))
            // .children(vec![
            //     kdl.clone(),
            //     kdl.clone(),
            //     kdl.clone().children(vec![
            //         kdl.clone().attr("child1"),
            //         kdl.clone().children(vec![
            //             kdl.clone(),
            //             kdl.clone().prop("nesting", v.str(&"ludicrous".to_string()))
            //         ])
            //     ])
            // ])
            .build()
    );
    // moves >_> that super lifting thing.
    // println!("{:#?}", &kdl.build());
    // println!("{:#?}", &kdl.build());
    // println!("{:#?}", &kdl.build());

    let mut hsm = HashSetMapBuilder::new();
    let mut hasher = DefaultHasher::new();
    let hash = hsm.insert(&mut hasher, "asd".to_string());
    println!("{hash:#?}");

    let mut hsm = HashSetMapBuilder::new();
    let mut hasher = &mut DefaultHasher::new();

    let hash = hsm.insert(hasher, "asd".to_string());
    println!("{hash:#?}");
    println!(
        "{:#?}",
        hsm.insert(&mut DefaultHasher::new(), "asdddd".to_string())
    );
    println!(
        "{:#?}",
        hsm.insert(&mut DefaultHasher::new(), "asdddd".to_string())
    );
    println!(
        "{:#?}",
        hsm.insert(&mut DefaultHasher::new(), "asdddd".to_string())
    );
    println!(
        "{:#?}",
        hsm.insert(&mut DefaultHasher::new(), "asdddd".to_string())
    );
    println!("{:#?}", hsm.build());

    // 0.6.0 ->
}

/*
ideas
    - unify str vs String
    - unify &mut self vs Self
    - nested structs pass in super and return self of super?
    - add a macro to create a builder
    - figure out what derive and annotation do
    - put all the strings in an owned vec, in the name field store the index
    */
