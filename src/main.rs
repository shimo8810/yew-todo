use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct ToDo {
    msg: String,
    done: bool,
}

#[derive(Clone, Properties, PartialEq)]
struct ToDoListProps {
    todos: Vec<ToDo>,
    onremove: Callback<usize>,
    ontoggle: Callback<usize>,
    ontoggle_all: Callback<()>,
}

#[derive(PartialEq, Properties, Clone)]
struct ToDoHeaderProps {
    onedit: Callback<String>,
}

#[function_component(ToDoHeader)]
fn todo_header(props: &ToDoHeaderProps) -> Html {
    let onkeypress = {
        let edit = props.onedit.clone();
        move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: HtmlInputElement = e.target_unchecked_into();
                let value = input.value();
                input.set_value("");
                edit.emit(value);
            }
        }
    };

    html! {
        <header>
            <h1> {"ToDo"} </h1>
            <input
                autofocus=true
                autocomplete="off"
                placeholder="what needs to be done?"
                class="new-todo"
                {onkeypress}
            />
        </header>
    }
}

#[function_component(ToDoList)]
fn todo_list(props: &ToDoListProps) -> Html {
    let ontoggle_all = {
        let ontoggle_all = props.ontoggle_all.clone();
        Callback::from(move |_| {
            ontoggle_all.emit(());
        })
    };

    html! {
        <section>
        <input
            type="checkbox"
            id="toggle-all"
            class="toggle-all"
            onclick={ontoggle_all}
        />
        <label for="toggle-all"></label>
        <ul class="todo-list">
        {
            props.todos.iter().enumerate().map(|(id, todo)| {
                let mut class = Classes::from("todo");

                if todo.done {
                    class.push("done");
                }
                let onremove = {
                    let onremove = props.onremove.clone();
                    Callback::from(move |_| onremove.emit(id))
                };

                let ontoggle = {
                    let ontoggle = props.ontoggle.clone();
                    Callback::from(move |_| {
                        ontoggle.emit(id);
                    })
                };

                html! {
                    <li class={class}>
                        <div class="todo-view">
                            <input type="checkbox" class="toggle" onclick={ontoggle} />
                            <label> {&todo.msg} </label>
                            <button class="destroy" onclick={onremove} value=1/>
                        </div>
                        <input type="hidden" class="edit"/>
                    </li>
                }
            }).collect::<Html>()
        }
        </ul>
        </section>
    }
}

#[function_component(App)]
fn app() -> Html {
    // 状態
    let todos = use_state(Vec::new);
    let toggle_all = use_state(|| true);

    // 追加するイベント
    let onedit = {
        let todos = todos.clone();
        let toggle_all = toggle_all.clone();
        Callback::from(move |msg: String| {
            toggle_all.set(true);
            let mut v = (*todos).clone();
            v.push(ToDo { msg, done: false });
            todos.set(v);
        })
    };

    // 要素の削除
    let onremove = {
        let todos = todos.clone();
        let toggle_all = toggle_all.clone();

        Callback::from(move |id: usize| {
            toggle_all.set(true);
            let mut v = (*todos).clone();
            v.remove(id);
            todos.set(v);
        })
    };

    // 要素の切り替え
    let ontoggle = {
        let todos = todos.clone();
        let toggle_all = toggle_all.clone();

        Callback::from(move |id: usize| {
            toggle_all.set(true);
            let mut v = (*todos).clone();
            v[id].done = !v[id].done;
            todos.set(v);
        })
    };

    //  すべてのタスクの切り替え
    let ontoggle_all = {
        let todos = todos.clone();
        Callback::from(move |_| {
            let mut v = (*todos).clone();
            for t in &mut v {
                t.done = *toggle_all;
            }
            todos.set(v);
            toggle_all.set(!*toggle_all);
        })
    };

    html! {
    <section class="todo-app">
        <ToDoHeader {onedit} />
        <ToDoList todos={(*todos).clone()} {onremove} {ontoggle} {ontoggle_all}/>
    </section>
    }
}

fn main() {
    yew::start_app::<App>();
}
