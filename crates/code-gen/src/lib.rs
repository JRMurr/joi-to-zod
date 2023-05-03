mod joi;

use genco::prelude::*;

pub fn gen() -> String {
    let react = &js::import("react", "React").into_default();
    let display = &js::import("./Display", "Display").into_default();
    let button_panel = &js::import("./ButtonPanel", "ButtonPanel").into_default();
    let calculate = &js::import("../logic/calculate", "calculate").into_default();

    let tokens = quote! {
        export default class App extends $react.Component {
            state = {
                total: null,
                next: null,
                operation: null,
            };

            handleClick = buttonName => {
                this.setState($calculate(this.state, buttonName));
            };

            render() {
                return (
                    <div className="component-app">
                        <$display value={this.state.next || this.state.total || "0"} />
                        <$button_panel clickHandler={this.handleClick} />
                    </div>
                );
            }
        }
    };

    tokens.to_string().expect("no error")
}
