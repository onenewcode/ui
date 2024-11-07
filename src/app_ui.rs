use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;
    
    TEXT_BIG = 12.0
    
    COLOR_UP_0 = #xFFFFFF00
    COLOR_DOWN_2 = #x00000022
    FONT_SIZE_H2 = 10.0
    
    SSPACING_0 = 0.0
    SSPACING_1 = 4.0
    SSPACING_2 = (SSPACING_1 * 2)
    SSPACING_3 = (SSPACING_1 * 3)
    SSPACING_4 = (SSPACING_1 * 4)
    
    SPACING_0 = {top: (SSPACING_0), right: (SSPACING_0), bottom: (SSPACING_0), left: (SSPACING_0)}
    SPACING_1 = {top: (SSPACING_1), right: (SSPACING_1), bottom: (SSPACING_1), left: (SSPACING_1)}
    SPACING_2 = {top: (SSPACING_2), right: (SSPACING_2), bottom: (SSPACING_2), left: (SSPACING_2)}
    SPACING_3 = {top: (SSPACING_3), right: (SSPACING_3), bottom: (SSPACING_3), left: (SSPACING_3)}
    SPACING_4 = {top: (SSPACING_4), right: (SSPACING_4), bottom: (SSPACING_4), left: (SSPACING_4)}
    // 记得加载中文字体文件，否则中文会出现乱码
    H2_TEXT_BOLD = {
        font_size: (FONT_SIZE_H2),
        font: {path: dep("crate://self/resources/SourceHanSerifCN-Light.ttf")}
    }
    
    H2_TEXT_REGULAR = {
        font_size: (FONT_SIZE_H2),
        font: {path: dep("crate://self/resources/SourceHanSerifCN-Light.ttf")}
    }
    
    TEXT_BOLD = {
        font_size: 10.0,
        font: {path: dep("crate://self/resources/SourceHanSerifCN-Light.ttf")}
    }
    
    TEXT_MONO = {
        font_size: 10.0,
        font: {path: dep("crate://self/resources/SourceHanSerifCN-Light.ttf")}
    }
    
    COLOR_PANEL_BG = (COLOR_DOWN_2)
    COLOR_TEXT_INPUT = (COLOR_DOWN_2)
    COLOR_LABEL = #xFFF9
    COLOR_DOWN_0 = #x00000000
    COLOR_DOWN_1 = #x00000011
    COLOR_DOWN_2 = #00000022
    COLOR_DOWN_3 = #x00000044
    COLOR_DOWN_4 = #x00000066
    COLOR_DOWN_5 = #x000000AA
    COLOR_DOWN_6 = #x000000CC 
    COLOR_White_1 = #FFFFFF
    COLOR_UP_0 = #xFFFFFF00
    COLOR_UP_1 = #xFFFFFF0A
    COLOR_UP_2 = #xFFFFFF10
    COLOR_UP_3 = #xFFFFFF20
    COLOR_UP_4 = #xFFFFFF40
    COLOR_UP_5 = #xFFFFFF66
    COLOR_UP_6 = #xFFFFFFCC
    COLOR_UP_FULL = #xFFFFFFFF
    HistoryDropDown = <DropDown dx:-924.5 dy:2947.3 dw:378.1 dh:54.0> {
        width: Fit
        padding: {top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 }

        draw_text: {
            text_style: <H2_TEXT_REGULAR> {},
            fn get_color(self) -> vec4 {
                return mix(
                    mix(
                        mix(
                            (#xFFF8),
                            (#xFFF8),
                            self.focus
                        ),
                        (#xFFFF),
                        self.hover
                    ),
                    (#x000A),
                    self.pressed
                )
            }
        }

        popup_menu: {
            menu_item: {
                indent_width: 10.0
                width: Fill,
                height: Fit
                padding: {top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 }

                draw_bg: {
                    color: #x48,
                    color_selected: #x6
                }
            }
        }

        draw_bg: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                self.get_bg(sdf);
                // triangle
                let c = vec2(self.rect_size.x - 10.0, self.rect_size.y * 0.5)
                let sz = 2.5;

                sdf.move_to(c.x - sz, c.y - sz);
                sdf.line_to(c.x + sz, c.y - sz);
                sdf.line_to(c.x, c.y + sz * 0.75);
                sdf.close_path();

                sdf.fill(mix(#FFFA, #FFFF, self.hover));

                return sdf.result
            }

            fn get_bg(self, inout sdf: Sdf2d) {
                sdf.rect(
                    0,
                    0,
                    self.rect_size.x,
                    self.rect_size.y
                )
                sdf.fill((COLOR_UP_0))
            }
        }
    }

    IconButton = <Button dx:-923.1 dy:2743.6 dw:372.4 dh:47.3> {
        draw_icon: {
            svg_file: (ICO_SAVE),
            fn get_color(self) -> vec4 {
                return mix(
                    mix(
                        (COLOR_UP_5),
                        (COLOR_UP_6),
                        self.hover
                    ),
                    (COLOR_UP_4),
                    self.pressed
                )
            }
        }
        icon_walk: {width: 7.5, height: Fit}
        draw_bg: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                return sdf.result
            }
        }
        padding: 9.0
        text: ""
    }
    
    AppUI = <View> {
                            
        flow: Overlay,
                            
                            
        width: Fill,
        height: Fill
                            
                            
        dock = <Dock> {
            height: Fill,
            width: Fill
                                    
            root = Splitter {
                axis: Horizontal,
                align: FromA(300.0),
                a: sidebar,
                b: main,
            }                       
            sidebar = Tab {
                name: ""
                kind:  Sidebar
            }
            main= Tab {
                name: ""
                kind: InputPanel
            }
                                                  
            InputPanel = <RectView> {
                height: Fill,
                width: Fill
                flow: Down,
                padding: 0.0
                draw_bg: {color: (COLOR_PANEL_BG)}
                <View> {
                    <View>{
                        flow: Down
                        <RoundedView>{
                            draw_bg:{
                                color: (COLOR_DOWN_2)
                                border_width: 1.0
                                border_color: #x00000044
                            }
                            margin:{top:0, left:0, right: 0, bottom:5}
                            align: {x:0.5},
                            padding: 2
                            width: Fill,
                            height: fill,
                            // 显示对话历史
                            llm_chat = <PortalList>{  
                                auto_tail:true,
                                width: Fill,
                                height: Fill,
                                margin: {top: 0},
                                AI = <TextInput> {
                                    width: Fill,
                                    height: Fill,
                                    margin: {top: 0.0, left: 20.0, bottom: 5.0, right: 0.0},
                                    text: "LLM Output"
                                    draw_text: {
                                        text_style: <TEXT_MONO> {font_size: (TEXT_BIG)}
                                    }
                                    draw_bg: {
                                        color: (#335)
                                    }
                                }
                                Human = <TextInput> {
                                    width: Fill,
                                    height: Fill,
                                    margin: {top: 0.0, left: 0.0, bottom: 5.0, right: 0.0},
                                    text: "LLM Output"
                                    draw_text: {
                                        text_style: <TEXT_MONO> {font_size: (TEXT_BIG)}
                                    }
                                    draw_bg: {
                                        color: (#353)
                                    }
                                }
                            }
                        }
                       // 对话输入框
                        chat = <TextInput> {
                            height: Fit,
                            width: Fill,
                            margin: {top: 0.0, left: 0.0, bottom: 0.0, right: 10.0},
                            empty_message: "Talk here"
                            draw_bg: {
                                color: (COLOR_TEXT_INPUT)
                            }
                            draw_text: {
                                text_style: {font_size: (TEXT_BIG)}
                                fn get_color(self) -> vec4 {
                                    return
                                    mix(
                                        mix(
                                            mix(
                                                #xFFFFFF55,
                                                #xFFFFFF88,
                                                self.hover
                                            ),
                                            #xFFFFFFCC,
                                            self.focus
                                        ),
                                        #xFFFFFF66,
                                        self.is_empty
                                    )
                                }
                            }
                        }
                    }
                }
            }
                                    
            Sidebar = <RectView> {
                draw_bg: {color: (COLOR_PANEL_BG)}
                height: Fill,
                width: Fill
                flow: Down
                <View> {
                    height: Fit,
                    width: Fill
                    flow: Right,
                    padding: {left: 10, right: 10, top: 10, bottom: 10},
                    new_chat = <Button> {
                        height: Fit,
                        width: Fit,
                        margin: {bottom: 0}
                        text: "New Chat"
                        draw_bg: {color: (COLOR_BUTTON)}
                        draw_text: {text_style: {font_size: (TEXT_BIG)}}
                    }
                    search_button= <Button> {
                        height: Fit,
                        width: Fit,
                        margin: {bottom: 0}
                        text: "Search"
                        draw_bg: {color: (COLOR_BUTTON)}
                        draw_text: {text_style: {font_size: (TEXT_BIG)}}
                    }
                    search = <TextInput> {
                        visible: false,
                        height: Fit,
                        width: Fill,
                        margin: {bottom: 0}
                        empty_message: "Search"
                        draw_bg: {
                            color: (COLOR_White_1)
                        }
                        draw_text: {
                            text_style: {font_size: (TEXT_BIG)}
                            fn get_color(self) -> vec4 {
                                return
                                mix(
                                    mix(
                                        mix(
                                            #xFFFFFF55,
                                            #xFFFFFF88,
                                            self.hover
                                        ),
                                        #xFFFFFFCC,
                                        self.focus
                                    ),
                                    #xFFFFFF66,
                                    self.is_empty
                                )
                            }
                        }
                    }
                }
                <View> {
                    height: Fill,
                    width: Fill
                    flow: Right,
                    padding: {left: 10, right: 10, top: 10, bottom: 10},
                    chat_record = <PortalList>{  
                        auto_tail:true,
                        width: Fill,
                        height: Fill,
                        margin: {top: 0},
                        HiatoryLabel =<View> {
                            label = <Label> {
                                margin: {top: 1}
                                draw_text: {
                                    text_style: <H2_TEXT_BOLD> {},
                                    color: (COLOR_White_1)
                                }
                                text: "replace me!"
                                animator: {
                                    hover = {
                                        default: off
                                        off = {
                                            from: {all: Forward {duration: 0.1}}
                                            apply: {
                                                draw_check: {hover: 0.0}
                                                draw_text: {
                                                    text_style: <H2_TEXT_BOLD> {},
                                                    color: (COLOR_UP_0)
                                                }
                                            }
                                        }
                                    }
                                }
                                
                            }
                            menu = <View> {
                                filter_type = <HistoryDropDown> {
                                    width: Fill
            
                                    labels: ["LowPass", "HighPass", "BandPass", "BandReject"]
                                    values: [LowPass, HighPass, BandPass, BandReject]
            
                                    draw_text: {
                                        text_style: <H2_TEXT_REGULAR> {},
                                        fn get_color(self) -> vec4 {
                                            return mix(
                                                mix(
                                                    mix(
                                                        (#x0008),
                                                        (#x0008),
                                                        self.focus
                                                    ),
                                                    (#x000F),
                                                    self.hover
                                                ),
                                                (#x000A),
                                                self.pressed
                                            )
                                        }
                                    }
            
                                    draw_bg: {
                                        fn pixel(self) -> vec4 {
                                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                            self.get_bg(sdf);
                                            // triangle
                                            let c = vec2(self.rect_size.x - 10.0, self.rect_size.y * 0.5)
                                            let sz = 2.5;
            
                                            sdf.move_to(c.x - sz, c.y - sz);
                                            sdf.line_to(c.x + sz, c.y - sz);
                                            sdf.line_to(c.x, c.y + sz * 0.75);
                                            sdf.close_path();
            
                                            sdf.fill(mix(#000A, #000F, self.hover));
            
                                            return sdf.result
                                        }
            
                                        fn get_bg(self, inout sdf: Sdf2d) {
                                            sdf.rect(
                                                0,
                                                0,
                                                self.rect_size.x,
                                                self.rect_size.y
                                            )
                                            sdf.fill((COLOR_UP_0))
                                        }
                                    }
            
                                    popup_menu: {
                                        menu_item: {
                                            indent_width: 10.0
                                            width: Fill,
                                            height: Fit
            
                                            padding: {left: (SSPACING_4), top: (SSPACING_2), bottom: (SSPACING_2), right: (SSPACING_2)}
                                        }
                                    }
            
                                }
                            }
                        }            
                    }
                }
                    
            }
            }
        }
}            