//! UI components for Dioxus: button, input, checkbox, select, slider, badge,
//! alert, card, divider, icon, spinner, loader, form, tabs.

pub mod alert;
pub mod badge;
pub mod button;
pub mod card;
pub mod checkbox;
pub mod divider;
pub mod form;
pub mod form_field;
pub mod icon;
pub mod input;
pub mod loader;
pub mod progress;
pub mod select;
pub mod slider;
pub mod spinner;
pub mod switch;
pub mod tabs;

pub use alert::Alert;
pub use badge::Badge;
pub use button::Button;
pub use card::{Card, CardBody, CardFooter, CardHeader};
pub use checkbox::{Checkbox, CheckboxState};
pub use divider::Divider;
pub use form::{
    FormCheckbox, FormError, FormGroup, FormInput, FormLabel, FormSelect, FormTextarea,
};
pub use form_field::{FormField, InputType, SelectOption, ValidationState};
pub use icon::{Icon, IconName};
pub use input::TextInput;
pub use loader::{ContentLoader, Loader, LoaderType, SkeletonLoader};
pub use progress::{Progress, ProgressIndicator};
pub use select::Select;
pub use slider::{Slider, SliderValue};
pub use spinner::{LoadingOverlay, Spinner};
pub use switch::Switch;
pub use tabs::Tabs;
