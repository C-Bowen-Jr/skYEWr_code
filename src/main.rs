use yew::prelude::*;

#[derive(PartialEq)]
struct SkuRCode {
    sku: String,
    action: String,
    quantity: i32,
}
enum ActionMessage {
    Stock,
    Sell,
    Retire,
    Restore,
    Inspect,
    GenerateQR,
}

impl Component for SkuRCode {
    type Message = ActionMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        SkuRCode {
            sku: "".to_string(),
            action: "".to_string(),
            quantity: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ActionMessage::Stock => self.quantity += 1,
            ActionMessage::Sell => self.quantity -= 1,
            _ => println!("Not implemented"),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
    <div class="view">
        <div class="product-grid">
            <img src="https://via.placeholder.com/200x200.png?text=Product+Image" alt="Product Image" />
            <img src="https://via.placeholder.com/200x200.png?text=Product+Image" alt="Product Image" />
            <img src="https://via.placeholder.com/200x200.png?text=Product+Image" alt="Product Image" />
            <img src="https://via.placeholder.com/200x200.png?text=Product+Image" alt="Product Image" />
            <img src="https://via.placeholder.com/200x200.png?text=Product+Image" alt="Product Image" />
            <img src="https://via.placeholder.com/200x200.png?text=Product+Image" alt="Product Image" />
            <img src="https://via.placeholder.com/200x200.png?text=Product+Image" alt="Product Image" />
            <img src="https://via.placeholder.com/200x200.png?text=Product+Image" alt="Product Image" />
            <img src="https://via.placeholder.com/200x200.png?text=Product+Image" alt="Product Image" />
            <img src="https://via.placeholder.com/200x200.png?text=Product+Image" alt="Product Image" />
            <img src="https://via.placeholder.com/200x200.png?text=Product+Image" alt="Product Image" />
            <img src="https://via.placeholder.com/200x200.png?text=Product+Image" alt="Product Image" />
            <img src="https://via.placeholder.com/200x200.png?text=Product+Image" alt="Product Image" />
            <img src="https://via.placeholder.com/200x200.png?text=Product+Image" alt="Product Image" />
            <img src="https://via.placeholder.com/200x200.png?text=Product+Image" alt="Product Image" />
            <img src="https://via.placeholder.com/200x200.png?text=Product+Image" alt="Product Image" />
            <img src="https://via.placeholder.com/200x200.png?text=Product+Image" alt="Product Image" />
            <img src="https://via.placeholder.com/200x200.png?text=Product+Image" alt="Product Image" />
            <img src="https://via.placeholder.com/200x200.png?text=Product+Image" alt="Product Image" />
            <img src="https://via.placeholder.com/200x200.png?text=Product+Image" alt="Product Image" />
        </div>
        <div class="tool-buffer"></div>
        <div class="tool-tray">
            <button onclick={link.callback(|_| ActionMessage::Sell)}>
                { "Sell" }
            </button>
            { &format!("{}*{}", self.sku, self.quantity) }
            <button onclick={link.callback(|_| ActionMessage::Stock)}>
                { "Stock" }
            </button>
        </div>
    </div>
        }
    }
}

fn main() {
    yew::Renderer::<SkuRCode>::new().render();
}
