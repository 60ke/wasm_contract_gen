Signature {
    constness: None,
    asyncness: None,
    unsafety: None,
    abi: None,
    fn_token: Fn,
    ident: Ident {
        ident: "Transfer",
        span: #0 bytes(1319..1327),
    },
    generics: Generics {
        lt_token: None,
        params: [],
        gt_token: None,
        where_clause: None,
    },
    paren_token: Paren,
    inputs: [
        Receiver(
            Receiver {
                attrs: [],
                reference: Some(
                    (
                        And,
                        None,
                    ),
                ),
                mutability: Some(
                    Mut,
                ),
                self_token: SelfValue,
            },
        ),
        Comma,
        Typed(
            PatType {
                attrs: [],
                pat: Ident(
                    PatIdent {
                        attrs: [],
                        by_ref: None,
                        mutability: None,
                        ident: Ident {
                            ident: "indexed_from",
                            span: #0 bytes(1339..1351),
                        },
                        subpat: None,
                    },
                ),
                colon_token: Colon,
                ty: Path(
                    TypePath {
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: [
                                PathSegment {
                                    ident: Ident {
                                        ident: "Address",
                                        span: #0 bytes(1353..1360),
                                    },
                                    arguments: None,
                                },
                            ],
                        },
                    },
                ),
            },
        ),
        Comma,
        Typed(
            PatType {
                attrs: [],
                pat: Ident(
                    PatIdent {
                        attrs: [],
                        by_ref: None,
                        mutability: None,
                        ident: Ident {
                            ident: "indexed_to",
                            span: #0 bytes(1362..1372),
                        },
                        subpat: None,
                    },
                ),
                colon_token: Colon,
                ty: Path(
                    TypePath {
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: [
                                PathSegment {
                                    ident: Ident {
                                        ident: "Address",
                                        span: #0 bytes(1374..1381),
                                    },
                                    arguments: None,
                                },
                            ],
                        },
                    },
                ),
            },
        ),
        Comma,
        Typed(
            PatType {
                attrs: [],
                pat: Ident(
                    PatIdent {
                        attrs: [],
                        by_ref: None,
                        mutability: None,
                        ident: Ident {
                            ident: "_value",
                            span: #0 bytes(1383..1389),
                        },
                        subpat: None,
                    },
                ),
                colon_token: Colon,
                ty: Path(
                    TypePath {
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: [
                                PathSegment {
                                    ident: Ident {
                                        ident: "U256",
                                        span: #0 bytes(1391..1395),
                                    },
                                    arguments: None,
                                },
                            ],
                        },
                    },
                ),
            },
        ),
    ],
    variadic: None,
    output: Default,
}