use yew::prelude::*;
use wasm_bindgen::prelude::*;
use std::fs;

#[derive(PartialEq)]
struct SkuRCode {
    sku: String,
    action: String,
    quantity: i32,
    products: Vec<String>,
}
enum ActionMessage {
    Stock,
    Sell,
    Retire,
    Restore,
    Inspect,
    GenerateQR,
    CloseQR,
}

#[wasm_bindgen(
    inline_js = "export function overlay_on() { document.getElementById('overlay').style.display = 'block'; }"
)]
extern "C" {
    fn overlay_on();
}
#[wasm_bindgen(
    inline_js = "export function overlay_off() { document.getElementById('overlay').style.display = 'none'; }"
)]
extern "C" {
    fn overlay_off();
}

impl SkuRCode {
    fn render_product(&self, product: String) -> Html {
        html! {
            <img src={ product } />
        }
    }
}
impl Component for SkuRCode {
    type Message = ActionMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut find_products: Vec<String> = vec![];
        let image_dir = "./products";
        /*for entry in fs::read_dir(image_dir).unwrap() {
            if let Ok(entry) = entry {
                if let Some(filetype) = entry.path().extension() {
                    if filetype == "jpg" || filetype == "png" {
                        find_products.push(entry.path()
                                                .to_str()
                                                .unwrap()
                                                .to_string())
                    }
                }
            }
        }*/
        //find_products.push("./products/LOVEPEACE.png".to_string());
        SkuRCode {
            sku: "".to_string(),
            action: "".to_string(),
            quantity: 0,
            products: find_products,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ActionMessage::Stock => self.quantity += 1,
            ActionMessage::Sell => self.quantity -= 1,
            ActionMessage::GenerateQR => overlay_on(),
            ActionMessage::CloseQR => overlay_off(),
            _ => println!("Not implemented"),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
    <div class="view">
        <div class="product-grid">
            <img src="products/DSCORPS.png" alt="Product Image" />
            {
                self.products.clone().into_iter().map(|sku| {
                    html!{<img src={sku} />}
                }).collect::<Html>()
            }
        </div>
        <div class="action-buffer"></div>
        <div class="action-tray">
            <div class="action-text" onclick={link.callback(|_| ActionMessage::GenerateQR)}>
                { &format!("{}*{}", self.sku, self.quantity) }
            </div>
            <button class="sell-button" onclick={link.callback(|_| ActionMessage::Sell)}>
                { "Sell" }
            </button>
            <button class="stock-button" onclick={link.callback(|_| ActionMessage::Stock)}>
                { "Stock" }
            </button>
            <button class="inspect-button" onclick={link.callback(|_| ActionMessage::Inspect)}>
                { "Inspect" }
            </button>
            <button class="retire-button" onclick={link.callback(|_| ActionMessage::Retire)}>
                { "Retire" }
            </button>
            <button class="restore-button" onclick={link.callback(|_| ActionMessage::Restore)}>
                { "Restore" }
            </button>
        </div>
        <div id="overlay" onclick={link.callback(|_| ActionMessage::CloseQR)}>
            <div class="overlay-qr">
                <img src="https://via.placeholder.com/200x200.png?text=QR+Code" alt="QR Code" />
            </div>
        </div>
    </div>
        }
    }
}

fn main() {
    yew::Renderer::<SkuRCode>::new().render();
}
