import axios from "axios";

const getBrowserId = () => {
    let id = localStorage.getItem("browser_id");
    if(!id){
        id = crypto.randomUUID();
        localStorage.setItem("browser_id", id);
    }
    return id;
}

const BROWSER_ID = getBrowserId();
const API_URL = "http://localhost:3000"
console.log(API_URL);

export const get_todos = async () => {
    const response = await axios.post(`${API_URL}/todos/all`, {
        username: BROWSER_ID
    });
    console.log(response.data);
    return response.data;
}

export const create_todo = async (title: string, content:string) => {
    const response = await axios.post(`${API_URL}/todos/create`, {
        title,
        content,
        username: BROWSER_ID
    });
    console.log(response.data);
    return response.data;
}
