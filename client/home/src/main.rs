use yew::prelude::*;

fn get_home_page() -> Html {
    html! {
        <div class="background">
            <div class="title_container">
                <div style="display: flex; margin-left: 30px;" class="can_select">
                    <img src="/assets/Senka_Icon.png" style="height: auto;"/>
                    <div style="
                        color: #EA5E27FF;
                        white-space: nowrap;
                        width: fit-content;
                        font-weight: bold;
                        font-size: 40px;">
                        {"Senka Panel"}
                    </div>
                </div>
                <ul class="horizontal_list">
                    <li class="list_tab list_tab_left_border">
                        <div class="list_tab senka_change_color text_center_container can_select">
                            <span>{"Login"}</span>
                        </div>
                    </li>
                    <li class="list_tab list_tab_left_border dropdown">
                        <div class="list_tab senka_change_color text_center_container can_select">
                            <a>{"More"}</a>
                        </div>
                        <div class="dropdown_content">
                            <div class="list_tab senka_change_color text_center_container text_border can_select">
                                <a>{"Source Code"}</a>                        
                            </div>
                            <div class="list_tab senka_change_color text_center_container text_border can_select">
                                <p>{"About Senka Panel"}</p>                
                            </div>
                        </div>
                    </li>
                </ul>
            </div>
            <div class="content_background">
                <div 
                    style="
                    background-color:#1f1f1f;
                    justify-content: space-between;
                    display: flex;
                    align-items: center;
                    width: 100%;
                    height: 80%;
                    border-bottom: gray 1px solid;">
                    <div style="
                        height: 100%;
                        width: 55%;
                        display: flex;
                        justify-content: right;">
                        <img src="/assets/Senka-release.png" 
                            style="
                            display: block;
                            margin-top: auto;
                            opacity: 0.8;
                            margin-bottom: auto;
                            margin-right: 50px;
                            height: 90%;
                            min-width: unset;
                            min-height: unset;"/>
                    </div>
                    <div style="
                        width: 45%;">
                        <p>
                            <span style="
                                font-size: 80px;
                                color: #EA5E27;">{"Senka"}</span>
                            <span style="
                                font-size: 80px;
                                color: white;">{"Panel"}</span>
                            <span>
                                <img src="/assets/Senka_Icon.png"/>
                            </span>
                        </p>
                        <p style="color: gray;">{"A friendly open-source personal server management platform"}</p>
                        <p>
                            <button class="senka_button">{"Quick Start"}</button>
                        </p>
                    </div>
                </div>
                <div>
                    <p style="
                        display: flex;
                        justify-content: center;
                        align-items: center;">
                        <span style="
                            color: gray;
                            font-size: 12px;">{"Powered by Oth1024"}</span>
                    </p>
                </div>
            </div>
        </div>
    }
}

#[function_component(Home)]
fn home() -> Html {
    get_home_page()
}

fn main() {
    yew::Renderer::<Home>::new().render();
}