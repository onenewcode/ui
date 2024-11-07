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
    COLOR_DOWN_2 = #x00000022
    COLOR_DOWN_3 = #x00000044
    COLOR_DOWN_4 = #x00000066
    COLOR_DOWN_5 = #x000000AA
    COLOR_DOWN_6 = #x000000CC
    
    COLOR_UP_0 = #xFFFFFF00
    COLOR_UP_1 = #xFFFFFF0A
    COLOR_UP_2 = #xFFFFFF10
    COLOR_UP_3 = #xFFFFFF20
    COLOR_UP_4 = #xFFFFFF40
    COLOR_UP_5 = #xFFFFFF66
    COLOR_UP_6 = #xFFFFFFCC
    COLOR_UP_FULL = #xFFFFFFFF
    PromptGroup = <RectView> {
        <DividerV> {}
        height: Fit,
        width: Fill,
        margin: {bottom: 10, top: 0}
        flow: Down,
        spacing: 0,
        padding: 0
        draw_bg: {
            instance hover: 0.0
            instance down: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let body = mix(mix(#53, #5c, self.hover), #33, self.down);
                sdf.fill_keep(body)
                return sdf.result
            }
        }
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: 0.5}}
                    ease: OutExp
                    apply: {
                        draw_bg: {hover: 0.0}
                        prompt = {draw_text: {hover: 0.0}}
                    }
                }
                on = {
                    ease: OutExp
                    from: {
                        all: Forward {duration: 0.2}
                    }
                    apply: {
                        draw_bg: {hover: 1.0}
                        prompt = {draw_text: {hover: 1.0}}
                    }
                }
            }
            down = {
                default: off
                off = {
                    from: {all: Forward {duration: 0.5}}
                    ease: OutExp
                    apply: {
                        draw_bg: {down: 0.0}
                        prompt = {draw_text: {down: 0.0}}
                    }
                }
                on = {
                    ease: OutExp
                    from: {
                        all: Forward {duration: 0.2}
                    }
                    apply: {
                        draw_bg: {down: 1.0}
                        prompt = {draw_text: {down: 1.0}}
                    }
                }
            }
        }
        prompt = <Label> {
            width: Fill
            draw_text: {
                text_style: <TEXT_BOLD> {},
                instance hover: 0.0
                instance down: 0.0
                fn get_color(self) -> vec4 {
                    return mix(mix(#xFFFA, #xFFFF, self.hover), #xFFF8, self.down);
                }
                wrap: Word,
            }
            text: ""
        }
    }
    
    ImageTile = <View> {
        width: Fill,
        height: Fit
        cursor: Hand
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: 0.5}}
                    ease: OutExp
                    apply: {
                        img = {draw_bg: {hover: 0.0}}
                    }
                }
                on = {
                    ease: OutExp
                    from: {
                        all: Forward {duration: 0.3}
                    }
                    apply: {
                        img = {draw_bg: {hover: 1.0}}
                    }
                }
            }
            down = {
                default: off
                off = {
                    from: {all: Forward {duration: 0.5}}
                    ease: OutExp
                    apply: {
                        img = {draw_bg: {down: 0.0}}
                    }
                }
                on = {
                    ease: OutExp
                    from: {
                        all: Forward {duration: 0.3}
                    }
                    apply: {
                        img = {draw_bg: {down: 1.0}}
                    }
                }
            }
        }
        
        img = <Image> {
            width: Fill,
            height: Fill
            min_width: 1920,
            min_height: 1080,
            fit: Horizontal,
            draw_bg: {
                instance hover: 0.0
                instance down: 0.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                    sdf.box(1, 1, self.rect_size.x - 2, self.rect_size.y - 2, 4.0)
                    let max_scale = vec2(0.92);
                    let scale = mix(vec2(1.0), max_scale, self.hover);
                    let pan = mix(vec2(0.0), (vec2(1.0) - max_scale) * 0.5, self.hover);
                    let color = self.get_color_scale_pan(scale, pan) + mix(vec4(0.0), vec4(0.1), self.down);
                    sdf.fill_keep(color);
                    sdf.stroke(
                        mix(mix(#x0000, #x0006, self.hover), #xfff2, self.down),
                        1.0
                    )
                    
                    return sdf.result
                }
            }
        }
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
                        history = <TextInput> {
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
                    }
                }
                    
                }
            }
        }
}            