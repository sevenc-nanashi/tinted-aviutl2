#[test]
fn merge_init() {
    let original = dedent::dedent!(
        r#"
        [Layout]
        ; ウィンドウ間のサイズ
        WindowSeparatorSize=2
        ; レイヤーの高さ(レイヤー編集)
        LayerHeight=20
        ; レイヤーの高さのメニュー(メニュー名,高さ) ※10個まで設定出来ます
        LayerHeightMenu0=小,12
        LayerHeightMenu1=中,20
        LayerHeightMenu2=大,36
        "#
    );

    assert_eq!(
        tinted_aviutl2::merge_style::merge_style(
            original,
            dedent::dedent!(
                r#"
                [Color]
                ; レイヤーの背景色
                LayerBackground=255,0,0
                "#
            )
        )
        .trim(),
        dedent::dedent!(
            r#"
            [Layout]
            ; ウィンドウ間のサイズ
            WindowSeparatorSize=2
            ; レイヤーの高さ(レイヤー編集)
            LayerHeight=20
            ; レイヤーの高さのメニュー(メニュー名,高さ) ※10個まで設定出来ます
            LayerHeightMenu0=小,12
            LayerHeightMenu1=中,20
            LayerHeightMenu2=大,36

            ; -- tinted-aviutl2-style-start --
            [Color]
            ; レイヤーの背景色
            LayerBackground=255,0,0
            ; -- tinted-aviutl2-style-end --
            "#
        )
        .trim()
    );
}

#[test]
fn merge_existing_color() {
    let original = dedent::dedent!(
        r#"
        [Font]
        ; フォント名
        Name=Yu Gothic

        [Color]
        ; レイヤーの背景色
        LayerBackground=255,255,255

        [Layout]
        ; ウィンドウ間のサイズ
        WindowSeparatorSize=2
        "#
    );

    assert_eq!(
        tinted_aviutl2::merge_style::merge_style(
            original,
            dedent::dedent!(
                r#"
                [Color]
                ; レイヤーの背景色
                LayerBackground=255,0,0
                "#
            )
        )
        .trim(),
        dedent::dedent!(
            r#"
            [Font]
            ; フォント名
            Name=Yu Gothic

            ; [Color]
            ; ; レイヤーの背景色
            ; LayerBackground=255,255,255
            ;
            [Layout]
            ; ウィンドウ間のサイズ
            WindowSeparatorSize=2

            ; -- tinted-aviutl2-style-start --
            [Color]
            ; レイヤーの背景色
            LayerBackground=255,0,0
            ; -- tinted-aviutl2-style-end --
            "#
        )
        .trim()
    );
}

#[test]
fn merge_with_existing_markers() {
    let original = dedent::dedent!(
        r#"
        [Layout]
        ; ウィンドウ間のサイズ
        WindowSeparatorSize=2
        ; -- tinted-aviutl2-style-start --
        [Color]
        ; レイヤーの背景色
        LayerBackground=255,255,255
        ; -- tinted-aviutl2-style-end --
        "#
    );

    assert_eq!(
        tinted_aviutl2::merge_style::merge_style(
            original,
            dedent::dedent!(
                r#"
                [Color]
                ; レイヤーの背景色
                LayerBackground=255,0,0
                "#
            )
        )
        .trim(),
        dedent::dedent!(
            r#"
            [Layout]
            ; ウィンドウ間のサイズ
            WindowSeparatorSize=2
            ; -- tinted-aviutl2-style-start --
            [Color]
            ; レイヤーの背景色
            LayerBackground=255,0,0
            ; -- tinted-aviutl2-style-end --
            "#
        )
        .trim()
    );
}
