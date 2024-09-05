use iced::{alignment, widget::{button, column, container, row, scrollable, text, text_input, Column}, Element, Length, Padding, Sandbox, Settings};
struct List {
    items: Vec<String>,
    input_value: String
}

#[derive(Debug, Clone)]
enum Message {
    InputValue(String),
	Submitted,
    DeleteItem(usize),
}

impl Sandbox for List {
	type Message = Message;
	
	/* Initialize your app */
	fn new() -> Self {
		Self {items: vec![], input_value: "".to_owned()}
	}
	
	/**
	* The title of the window. It will show up on the top of your application window.
	*/
	fn title(&self) -> String {
		String::from("Grocery List App")
	}
	
	fn update(&mut self, message: Self::Message) {
        match message {
            Message::InputValue(value) => self.input_value = value,
            Message::Submitted => {
                self.items.push(self.input_value.clone());
                self.input_value = String::default(); // Clear the input value
            }
            Message::DeleteItem(item) => {
                self.items.remove(item);
            },        
        }
	}
	
	fn view(&self) -> Element<Self::Message> {
		container(
            column!(
                self.view_items(),
                row!(
                    text_input("Input grocery item", &self.input_value)
                    .on_input(|value| Message::InputValue(value))
                    .on_submit(Message::Submitted), 
    
                    button("Submit")
                    .on_press(Message::Submitted)
                )
                .spacing(30)
                .padding(Padding::from(30))
            )
            .align_items(iced::Alignment::Center)
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .into()
	}

    fn theme(&self) -> iced::Theme {
		iced::Theme::Dark
	}
}

impl List {
    fn view_items(&self) -> Element<'static, Message> {
        let mut column = Column::new()
        .spacing(20)
        .align_items(iced::Alignment::Center)
        .width(Length::Fill);
        
        for i in 0..self.items.len() {
            column = column.push(self.grocery_item(i));
        }
        
        scrollable(
            container(
                column
            )
        )
        .height(250.0)
        .width(300)
        .into()
    }

    fn grocery_item(&self, index: usize) -> Element<'static, Message> {
        row!(
            text(self.items[index].clone()),
            button("Mark as Done")
            .on_press(Message::DeleteItem(index))
        )
        .align_items(iced::Alignment::Center)
        .spacing(30)
        .into()
    }
}

fn main() -> iced::Result {
	List::run(Settings::default())
}