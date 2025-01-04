use std::fmt::format;

use useless_utils::unsort::{unsort, unsort_undeterministic, Algorithm};
use yew::{classes, function_component, html, Html, Properties};
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
struct Props {
    list: Vec<i32>,
    alg: Algorithm,
    undeterministic: bool,
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/unsort")]
    Unsort,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Unsort => html!(<Unsort />),
    }
}

#[function_component]
fn SortDemo(props: &Props) -> Html {
    let mut new_props = props.clone();
    match props.undeterministic {
        true => unsort_undeterministic(new_props.alg, new_props.list.as_mut_slice()),
        false => unsort(new_props.alg, new_props.list.as_mut_slice()),
    }
    html!(<article>
    {format!("{:?}", props.list)}
    {" --> "}
    {format!("{:?}", new_props.list)}
    </article>)
}

fn merge_sort<T: Ord + Clone>(arr: &mut [T]) -> u32 {
    let len = arr.len();
    if len <= 1 {
        return 1;
    }

    let mid = len / 2;

    // Split the array into two halves
    let mut left = arr[0..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    // Recursively sort both halves
    let l = merge_sort(&mut left);
    let r = merge_sort(&mut right);

    // Merge the sorted halves back into the original array
    merge(&left, &right, arr) + l + r
}

fn merge<T: Ord + Clone>(left: &[T], right: &[T], arr: &mut [T]) -> u32 {
    let mut left_idx = 0;
    let mut right_idx = 0;
    let mut arr_idx = 0;

    let mut count = 0;

    // Merge elements from left and right arrays in sorted order
    while left_idx < left.len() && right_idx < right.len() {
        if left[left_idx] <= right[right_idx] {
            arr[arr_idx] = left[left_idx].clone();
            left_idx += 1;
        } else {
            arr[arr_idx] = right[right_idx].clone();
            right_idx += 1;
        }
        arr_idx += 1;
        count += 1
    }

    // Copy remaining elements from left array, if any
    while left_idx < left.len() {
        arr[arr_idx] = left[left_idx].clone();
        left_idx += 1;
        arr_idx += 1;
    }

    // Copy remaining elements from right array, if any
    while right_idx < right.len() {
        arr[arr_idx] = right[right_idx].clone();
        right_idx += 1;
        arr_idx += 1;
    }
    count
}
#[function_component]
fn Unsort() -> Html {
    let mut list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    unsort_undeterministic(Algorithm::Merge, list.as_mut_slice());
    let count = merge_sort(&mut list);
    html! {<main class={classes!("container")}>
    <h1>{"Unsort \"Documentation\""}</h1>
    <p>{"Unsort is an extremely versatile utility that lets you sort a list into the worst time scenario with a given sorting algorithm. With the undeterministic feature, The unsort functions will be able to produce most of the valid cases."}</p>
    <p>{"This is a standard unsorted list with the merge sort algorithm without any special features"}</p>
    <SortDemo alg={Algorithm::Merge} undeterministic={false} list={vec![1,2,3,4,5,6,7,8,9,10]} />
    <p>{"This is the same list but unsorted with our flagship \"undeterministic\" feature. It's different! Try refreshing the page and see what happens."}</p>
    <SortDemo alg={Algorithm::Merge} undeterministic={true} list={vec![1,2,3,4,5,6,7,8,9,10]} />
    <p>{"Of course, we also support a variety of algorithms to work in any scenario. We also have heap sort, bubble sort and quick sort with 3 different pivot presets."}</p>
    {count} {format!("{:?}", list)}
    </main>}
}

#[function_component]
fn Home() -> Html {
    html! {<main class={classes!("container")}>
    <h1>{"Useless Utils \"Documentation\""}</h1>
    <p>{"This is useless utils. Rust's most versatile and powerful library that you need. To learn how to use our single module, click on the link below."}</p>
    <Link<Route> to={Route::Unsort}>{"Unsort \"Documentation\""}</Link<Route>>
    </main>}
}

#[function_component]
fn App() -> Html {
    html! {<>
    <BrowserRouter>
        <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
    </BrowserRouter>
    </>}
}

fn main() {
    println!("Hello, world!");
    yew::Renderer::<App>::new().render();
}
