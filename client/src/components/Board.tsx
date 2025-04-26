import { useEffect, useState } from "react";
import { get_todos, create_todo } from "../lib/api";

const Board = () => {
    const [todos, setTodos] = useState([]);
    const [text, setText] = useState("");
    const [addTask, setAddTask] = useState(false);
    const [submitted, setSubmitted] = useState(true);
    useEffect(() => {
        const refreshTodos = async () => {
            if (submitted) {
                const response = await get_todos();
                setTodos(response);
                setSubmitted(false);
                console.log(todos);
            }
        }
        refreshTodos();
    }, [submitted, setSubmitted]);

    const toggleAdd = (e) => {
        e.preventDefault();
        setAddTask(!addTask);
    }

    const handleSubmit = () => {
        const lines = text.split('\n');
        const title = lines[0];
        const content = lines.slice(1).join('\n');
        const response = create_todo(title, content);
        console.log(response);
        setSubmitted(true);
        setAddTask(false);
    }

    return (
        <>
            <div className="w-screen flex flex-col justify-start items-start">
                <div className="w-full mt-5 ml-5"><h1 className="text-3xl text-white">Your Todos</h1></div>
                {todos.length >= 0 ? <>
                    <div className="mt-5 ml-5 flex flex-col">
                        {todos?.map((item, index) => (<ul key={index} className="text-xl text-white">{index + 1}{". "}{item.title}</ul>))}
                    </div>
                </> : <><p className="ml-5 mt-5 text-xl text-white">You have not created any todos yet!</p></>}
                {addTask ? <><div className="mt-5 ml-5 w-[30vw] h-32">
                    <textarea className="h-[90%] w-[90%] bg-black text-sm text-white px-2 py-2 border border-textgray rounded-md resize-none focus:outline-white focus:outline-1" placeholder="Add new task" onChange={(e) => setText(e.target.value)}></textarea>
                    <div className="w-[90%] pr-2 flex items-end justify-end gap-3">
                        <button className="text-textgray text-sm px-2 py-1 hover:text-white" onClick={() => setAddTask(false)}>Close</button>
                        <button className="text-black bg-white text-sm px-2 py-1 rounded-sm hover:brightness-75" onClick={handleSubmit}>Add +</button>
                    </div>
                </div></> : <></>}
                {addTask ? <></> : <><div className="mt-5 ml-5 text-textgray hover:text-white"><button onClick={(e) => toggleAdd(e)}>Add Task +</button></div></>}
            </div>
        </>
    )
}

export default Board;