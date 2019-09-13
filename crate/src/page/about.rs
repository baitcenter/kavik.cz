use crate::generated::css_classes::C;
use seed::prelude::*;
use seed::*;
use super::{view_header, view_footer, Page};

pub fn view<Ms: 'static>() -> Vec<Node<Ms>> {
    vec![
        view_content().els(),
        view_header(Page::About).els(),
        view_footer().els(),
    ]
    .into_iter()
    .flatten()
    .collect()
}

pub fn view_content<Ms: 'static>() -> impl View<Ms> {
    vec![
        // Photo section
        section![
            class![
                C.w_screen,
                C.h_690px,
                C.bg_blue_10,
                // sm__
                C.sm__h_790px,
            ],
            // Small photo background container
            div![
                class![
                    C.absolute,
                    C.top_0,
                    C.inset_x_0,
                    C.flex,
                    C.justify_center,
                    // sm__
                    C.sm__hidden,
                ],
                // Small photo background
                div![
                    class![
                        C.w_xs,
                        C.h_300px,
                        C.bg_gray_1,
                    ]
                ],
            ],
            // Large photo background
            div![
                class![
                    C.absolute,
                    C.top_0,
                    C.inset_x_0,
                    C.h_320px,
                    C.rounded_bl_140px,
                    C.bg_gray_1,
                    // sm__
                    C.sm__h_420px,
                ],
            ],
            // Gear
            img![
                class![
                    C.absolute
                    C.left_full,
                    C._ml_40,
                    C._mt_56,
                    C.w_md,
                    C.blur,
                    // sm__
                    C.sm___mt_64,
                    C.sm__w_750px,
                ],
                attrs!{
                    At::Src => "/static/images/gear.svg"
                }
            ],
        ],
        // Developer section
        section![
            class![
                C.relative,
                C._mt_260px,
                C.h_1480px,
                C.pt_px,
                C.rounded_tr_140px,
                C.bg_gray_1,
                C.overflow_x_hidden,
                // sm__
                C.sm__h_1890px
            ],
            // Right background
            div![
                class![
                    C.absolute,
                    C.right_0,
                    C.inset_y_0,
                    C.w_76,
                    C.bg_yellow_4
                    // sm__
                    C.sm__w_132,
                ]
            ],
            // I, developer
            h2![
                class![
                    C.relative,
                    C.mt_32,
                    C.mb_16,
                    C.text_center,
                    C.font_monospace,
                    C.font_bold,
                    C.text_40,
                    C.text_blue_10
                    // sm__
                    C.sm__mt_40
                    C.sm__mb_24
                    C.sm__text_70
                ],
                "I, developer"
            ],
            ul![
                class![
                    C.relative,
                    C.text_gray_8,
                ],
                li![
                    class![
                        C.w_76,
                        C.pl_2,
                        C.pr_5,
                        C.py_10,
                        C.bg_gray_1,
                        C.flex,
                        C.flex_no_wrap,
                        // sm__
                        C.sm__pl_4,
                        C.sm__py_16,
                        C.sm__w_132,
                    ],
                    div![
                        class![
                            C.text_blue_6,
                            C.mr_1,
                            // sm__
                            C.sm__mr_2,
                        ],
                        "▶\u{fe0e}"
                    ],
                    div![
                        "I was working as a ",
                        span![
                            class![
                                C.font_bold,
                            ],
                            "backend"
                        ],
                        " developer in a bank and for some startups and agencies last years."
                    ]
                ],
                li![
                    class![
                        C.flex,
                        C.justify_end
                    ],
                    div![
                        class![
                            C.w_76,
                            C.pl_2,
                            C.pr_5,
                            C.py_10,
                            C.flex,
                            C.flex_no_wrap,
                            // sm__
                            C.sm__pl_4,
                            C.sm__py_16,
                            C.sm__w_132,
                        ],
                        div![
                            class![
                                C.text_blue_6,
                                C.mr_1,
                                // sm__
                                C.sm__mr_2,
                            ],
                            "▶\u{fe0e}"
                        ],
                        div![
                            "I'm also coming back to ",
                            span![
                                class![
                                    C.font_bold,
                                ],
                                "frontend"
                            ],
                            " because it's finally possible to write reliable web apps and I want to make more use of my ",
                            span![
                                class![
                                    C.font_bold,
                                ],
                                "artistic"
                            ],
                            " self."
                        ]
                    ]
                ],
                li![
                    class![
                        C.w_76,
                        C.pl_2,
                        C.pr_4,
                        C.py_10,
                        C.flex,
                        C.flex_no_wrap,
                        C.bg_gray_1
                        // sm__
                        C.sm__pl_4,
                        C.sm__py_16,
                        C.sm__w_132,
                    ],
                    div![
                        class![
                            C.text_blue_6,
                            C.mr_1,
                            // sm__
                            C.sm__mr_2,
                        ],
                        "▶\u{fe0e}"
                    ],
                    div![
                        "People make mistakes. That's why I setup linters, formatters, CI pipelines, etc., as ",
                        span![
                            class![
                                C.font_bold,
                            ],
                            "strict"
                        ],
                        " as possible and I want to write in ",
                        span![
                            class![
                                C.font_bold,
                            ],
                            "Rust"
                        ],
                        ". Ideal tools \"bully\" me and don't have any configuration options."
                    ]
                ],
                li![
                    class![
                        C.flex,
                        C.justify_end
                    ],
                    div![
                        class![
                            C.w_76,
                            C.pl_2,
                            C.pr_5,
                            C.py_10,
                            C.flex,
                            C.flex_no_wrap,
                            // sm__
                            C.sm__pl_4,
                            C.sm__py_16,
                            C.sm__w_132,
                        ],
                        div![
                            class![
                                C.text_blue_6,
                                C.mr_1,
                                // sm__
                                C.sm__mr_2,
                            ],
                            "▶\u{fe0e}"
                        ],
                        div![
                            "I often learn from ",
                            span![
                                "packtpub.com"
                            ],
                            ". And I recommend to read book ",
                            span![
                                "Domain Modeling Made Functional"
                            ],
                            "."
                        ]
                    ]
                ],
            ]
        ],
        // Designer section
        section![
            class![
                C.relative,
                C._mt_260px,
                C.h_1160px,
                C.pt_px,
                C.rounded_tr_140px,
                C.bg_blue_10,
                C.overflow_hidden,
                // sm__
                C.sm__h_1580px,
            ],
            // Circles
            div![
                class![
                    C.absolute,
                    C.left_1of2,
                    C._mt_12,
                    C._ml_545px,
                    C.w_1090px,
                    C.h_1090px,
                    C.opacity_10,
                    // sm__
                    C.sm___ml_820px,
                    C.sm__w_1640px,
                    C.sm__h_1580px,
                ],
                div![
                    class![
                        C.absolute,
                        C.left_0,
                        C.bottom_0,
                        C.w_570px,
                        C.h_570px,
                        C.rounded_full,
                        C.border_yellow_4,
                        C.border_2
                        // sm__
                        C.sm__w_860px,
                        C.sm__h_860px,
                    ]
                ],
                div![
                    class![
                        C.absolute,
                        C.right_0,
                        C.top_0,
                        C.w_570px,
                        C.h_570px,
                        C.rounded_full,
                        C.border_yellow_4,
                        C.border_2,
                        // sm__
                        C.sm__w_860px,
                        C.sm__h_860px,
                    ]
                ],
            ],
            // I, designer
            div![
                class![
                    C.relative,
                    C.mt_32,
                    C.h_24,
                    C.bg_blue_10,
                    C.flex,
                    C.items_center,
                    C.justify_center,
                    // sm__
                    C.sm__mt_48,
                    C.sm__h_40,
                ],
                h2![
                    class![
                        C.text_center,
                        C.font_display,
                        C.font_semibold,
                        C.text_40,
                        C.text_yellow_4,
                        // sm__
                        C.sm__text_70,
                    ],
                    "I, designer"
                ],
            ],
            ul![
                class![
                    C.relative,
                    C.mt_16,
                    C.max_w_md,
                    C.mx_auto,
                    C.pl_1,
                    C.text_blue_1,
                    C.flex,
                    C.flex_col,
                    // sm__
                    C.sm__mt_24,
                    C.sm__max_w_3xl,
                    C.sm__pl_5,
                    C.sm__pr_2,
                ],
                li![
                    class![
                        C.w_76,
                        C.flex,
                        C.flex_no_wrap
                        // sm__
                        C.sm__w_132,
                    ],
                    div![
                        class![
                            C.text_yellow_4,
                            C.mr_1,
                            // sm__
                            C.sm__mr_2,
                        ],
                        "▶\u{fe0e}"
                    ],
                    div![
                        "I've designed logos, my resume and this website in ",
                        span![
                            class![
                                C.font_bold,
                            ],
                            "Affinity Designer"
                        ],
                        "."
                    ]
                ],
                li![
                    class![
                        C.flex,
                        C.justify_end,
                    ],
                    div![
                        class![
                            C.mt_16,
                            C.ml_5,
                            C.w_64,
                            C.flex,
                            C.flex_no_wrap,
                            // sm__
                            C.sm__mt_24,
                            C.sm__ml_8,
                            C.sm__w_md,
                        ],
                        div![
                            class![
                                C.text_yellow_4,
                                C.mr_1,
                                // sm__
                                C.sm__mr_2,
                            ],
                            "▶\u{fe0e}"
                        ],
                        div![
                            "I'll use ",
                            span![
                                class![
                                    C.font_bold,
                                ],
                                "Figma"
                            ],
                            " for my next website design."
                        ]
                    ]
                ],
                li![
                    class![
                        C.mt_16,
                        C.w_76,
                        C.flex,
                        C.flex_no_wrap,
                        // sm__
                        C.sm__mt_24,
                        C.sm__w_132,
                    ],
                    div![
                        class![
                            C.text_yellow_4,
                            C.mr_1,
                            // sm__
                            C.sm__mr_2,
                        ],
                        "▶\u{fe0e}"
                    ],
                    div![
                        "I have some experience with ",
                        span![
                            class![
                                C.font_bold,
                            ],
                            "Adobe XD"
                        ],
                        ", ",
                        span![
                            class![
                                C.font_bold,
                            ],
                            "Krita"
                        ],
                        " and ",
                        span![
                            class![
                                C.font_bold,
                            ],
                            "Rhino3D"
                        ],
                        "."
                    ]
                ],
                li![
                    class![
                        C.flex,
                        C.justify_end,
                    ],
                    div![
                        class![
                            C.mt_16,
                            C.ml_6,
                            C.w_76,
                            C.flex,
                            C.flex_no_wrap,
                            // sm__
                            C.sm__mt_24,
                            C.sm__ml_8,
                            C.sm__w_md,
                        ],
                        div![
                            class![
                                C.text_yellow_4,
                                C.mr_1,
                                // sm__
                                C.sm__mr_2,
                            ],
                            "▶\u{fe0e}"
                        ],
                        div![
                            "I recommend to check ",
                            span![
                                "refactoringui.com"
                            ],
                            ". I've bought their book and I use their ",
                            span![
                                "TailwindCSS"
                            ],
                            " in my projects."
                        ]
                    ]
                ],
            ]
        ],
        // Human section
        section![
            class![
                C.relative,
                C._mt_260px,
                C.h_1580px,
                C.pt_px,
                C.rounded_tr_140px,
                C.bg_blue_6,
                C.overflow_hidden,
                C.flex,
                C.flex_col,
                C.items_center,
                // sm__
                C.sm__h_2330px,
            ],
            // I, human
            h2![
                class![
                    C.mt_24,
                    C.font_ordinary,
                    C.font_bold,
                    C.text_40,
                    C.text_blue_2,
                    // sm__
                    C.sm__mt_48,
                    C.sm__text_70
                ],
                "I, human"
            ],
            // Personal life
            div![
                class![
                    C.relative,
                    C.mt_20,
                    C.pt_3,
                    C.pb_16,
                    C.px_12,
                    C.bg_blue_10,
                    C.rounded_tr_110px,
                    // sm__
                    C.sm__mt_40,
                    C.sm__pb_32
                ],
                // Extended background
                div![
                    class![
                        C.absolute,
                        C.left_0,
                        C.inset_y_0,
                        C._left_50vw,
                        C.w_50vw,
                        C.bg_blue_10,
                    ]
                ],
                // Content container
                div![
                    class![
                        C.w_xs,
                        C.relative,
                        // sm__
                        C.sm__w_132,
                    ],
                    h3![
                        class![
                            C.ml_8,
                            C.mt_8,
                            C.font_display,
                            C.font_thin,
                            C.text_35,
                            C.text_blue_3,
                            // sm__
                            C.sm__mt_16,
                            C.sm__text_60
                        ],
                        "Personal life"
                    ],
                    ul![
                        class![
                            C.mt_8,
                            C.ml_1,
                            C.text_blue_1,
                            // sm__
                            C.sm__mt_12,
                        ],
                        li![
                            class![
                                C.flex,
                                C.flex_no_wrap
                            ],
                            div![
                                class![
                                    C.text_yellow_4,
                                    C.mr_1,
                                    // sm__
                                    C.sm__mr_2
                                ],
                                "▶\u{fe0e}"
                            ],
                            div![
                                "I'm INTJ. When I'm not ",
                                span![
                                    class![
                                        C.font_bold,
                                    ],
                                    "creating"
                                ],
                                " something, I usually read or go to gym."
                            ]
                        ],
                        li![
                            class![
                                C.mt_10,
                                C.flex,
                                C.flex_no_wrap,
                                // sm__
                                C.sm__mt_16
                            ],
                            div![
                                class![
                                    C.text_yellow_4,
                                    C.mr_1,
                                    // sm__
                                    C.sm__mr_2
                                ],
                                "▶\u{fe0e}"
                            ],
                            div![
                                "I like to spend my vacation at the cottage - hiking, cycling, driving, etc."
                            ]
                        ]
                    ]
                ]
            ],
            // Work life
            div![
                class![
                    C.relative,
                    C.mt_12,
                    C.pt_3,
                    C.pb_16,
                    C.px_12,
                    C.bg_blue_10,
                    C.rounded_tl_110px,
                    // sm__
                    C.sm__mt_24,
                    C.sm__pb_32
                ],
                // Extended background
                div![
                    class![
                        C.absolute,
                        C._right_50vw,
                        C.inset_y_0,
                        C.w_50vw,
                        C.bg_blue_10,
                    ]
                ],
                // Content container
                div![
                    class![
                        C.relative,
                        C.w_xs,
                        // sm__
                        C.sm__w_132,
                    ],
                    h3![
                        class![
                            C.mt_8,
                            C.mr_8,
                            C.text_right,
                            C.font_display,
                            C.font_thin,
                            C.text_35,
                            C.text_blue_3,
                            // sm__
                            C.sm__mt_16,
                            C.sm__text_60
                        ],
                        "Work life"
                    ],
                    ul![
                        class![
                            C.mt_8,
                            C.ml_1,
                            C.text_blue_1,
                            // sm__
                            C.sm__mt_12,
                        ],
                        li![
                            class![
                                C.flex,
                                C.flex_no_wrap
                            ],
                            div![
                                class![
                                    C.text_yellow_4,
                                    C.mr_1,
                                    // sm__
                                    C.sm__mr_2
                                ],
                                "▶\u{fe0e}"
                            ],
                            div![
                                "I'm ",
                                span![
                                    class![
                                        C.font_bold,
                                    ],
                                    "more productive"
                                ],
                                " when I'm working ",
                                span![
                                    class![
                                        C.font_bold,
                                    ],
                                    "remotely"
                                ],
                                "."
                            ]
                        ],
                        li![
                            class![
                                C.mt_10,
                                C.flex,
                                C.flex_no_wrap,
                                // sm__
                                C.sm__mt_16
                            ],
                            div![
                                class![
                                    C.text_yellow_4,
                                    C.mr_1,
                                    // sm__
                                    C.sm__mr_2
                                ],
                                "▶\u{fe0e}"
                            ],
                            div![
                                "I like to ",
                                span![
                                    class![
                                        C.font_bold,
                                    ],
                                    "help"
                                ],
                                " people (not only on GitHub) and to mentor juniors."
                            ]
                        ],
                        li![
                            class![
                                C.mt_10,
                                C.flex,
                                C.flex_no_wrap,
                                // sm__
                                C.sm__mt_16
                            ],
                            div![
                                class![
                                    C.text_yellow_4,
                                    C.mr_1,
                                    // sm__
                                    C.sm__mr_2
                                ],
                                "▶\u{fe0e}"
                            ],
                            div![
                                "I'd rather think about your project for free in a gym than sit and wait for ideas.",
                                br![],
                                "I also recommend to read ",
                                span![
                                    "You don’t need standup"
                                ],
                                "."
                            ]
                        ]
                    ]
                ]
            ]
        ],
        // Did you know section
        section![
            class![
                C.relative,
                C._mt_260px,
                C.h_580px,
                C.pt_px,
                C.rounded_tr_140px,
                C.bg_gray_1,
                C.overflow_hidden,
                C.flex,
                C.flex_col,
                C.items_center,
                // sm__
                C.sm__h_980px,
            ],
            h2![
                class![
                    C.mt_32,
                    C.mb_16,
                    C.font_display,
                    C.font_semibold,
                    C.text_40,
                    C.text_gray_5,
                    // sm__
                    C.sm__mt_56,
                    C.sm__mb_32,
                    C.sm__text_70,
                ],
                "Did you know..."
            ],
            ul![
                class![
                    C.ml_12,
                    C.w_xs,
                    C.text_gray_8,
                    // sm__
                    C.sm__w_md,
                ],
                li![
                    class![
                        C.flex,
                        C.flex_no_wrap
                    ],
                    div![
                        class![
                            C.text_blue_6,
                            C.mr_1,
                            // sm__
                            C.sm__mr_2,
                        ],
                        "▶\u{fe0e}"
                    ],
                    div![
                        "I programmed a real football cannon."
                    ]
                ],
                li![
                    class![
                        C.mt_10,
                        C.flex,
                        C.flex_no_wrap
                        // sm__
                        C.sm__mt_20
                    ],
                    div![
                        class![
                            C.text_blue_6,
                            C.mr_1,
                        ],
                        "▶\u{fe0e}"
                    ],
                    div![
                        "I jumped off a plane",
                        br![],
                        "and a bridge."
                    ]
                ],
            ]
        ],
        // Want to meet section
        section![
            class![
                C.relative,
                C.h_690px,
                C.bg_blue_10,
                C.rounded_br_140px,
                C.flex,
                C.flex_col,
                C.items_center,
                // sm__
                C.sm__h_930px,
            ],
            div![
                class![
                    C.relative,
                    C._mt_6,
                    C.flex,
                    C.justify_center,
                    // sm__
                    C.sm___mt_10
                ],
                img![
                    class![
                        C.relative,
                        C.block,
                        C.ml_10vw,
                        C.w_265px,
                        C.object_contain,
                        C.h_full,
                        // sm__
                        C.sm__w_385px,
                    ],
                    attrs!{
                        At::Src => "/static/images/photo_2.jpg",
                    }
                ],
            ],
            ul![
                class![
                    C.mt_16,
                    C.w_xs,
                    C.text_blue_1,
                    // sm__
                    C.sm__mt_24,
                    C.sm__w_132,
                    C.sm__pl_2
                ],
                li![
                    class![
                        C.flex,
                        C.flex_no_wrap
                    ],
                    div![
                        class![
                            C.text_yellow_4,
                            C.mr_1,
                            // sm__
                            C.sm__mr_2
                        ],
                        "▶\u{fe0e}"
                    ],
                    div![
                        "Want to meet somewhere in ",
                        span![
                            class![
                                C.font_bold
                            ],
                            "Prague"
                        ],
                        "? Is there good coffee, tea, sushi or some spicy food? Ok! ",
                        span![
                            "martin@kavik.cz"
                        ]
                    ]
                ],
            ]
        ],
        // Resume section
        section![
            class![
                C.h_48,
                C.flex,
                C.justify_center,
                C.items_center,
                // sm__
                C.sm__h_72
            ],
            a![
                class![
                    C.mt_6,
                    C.text_19,
                    C.text_gray_10,
                    C.flex,
                    C.whitespace_no_wrap,
                    // sm__
                    C.sm__text_29,
                ],
                "Download my\u{00A0}",
                span![
                    class![
                        C.font_semibold
                    ],
                    "Resume"
                ],
                span![
                    class![
                        C.font_semibold,
                        C.text_gray_5
                    ],
                    ".pdf"
                ],
                img![
                    class![
                        C._mt_1,
                        C.ml_1,
                        C.w_12
                        // sm__
                        C.sm__w_16
                    ],
                    attrs!{
                        At::Src => "/static/images/download.svg"
                    }
                ],
            ]
        ],
    ]
}
