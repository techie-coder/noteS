import { useEffect, useState } from "react";
import { get_todos, create_todo, delete_todo } from "../lib/api";
import { DropIndicator } from "./DropIndicator";
import fireIcon from "../assets/fire2.png";
import deleteIcon from "../assets/trash.png";


const Board = () => {
    const [todos, setTodos] = useState([]);
    const [text, setText] = useState("");
    const [addTask, setAddTask] = useState(false);

    useEffect(() => {
        async function fn() {
            setTodos(await get_todos());
        }
        fn();
    }, []);

    const toggleAdd = (e) => {
        e.preventDefault();
        setAddTask(!addTask);
    }

    const handleSubmit = async () => {
        const lines = text.split('\n');
        const title = lines[0];
        const content = lines.slice(1).join('\n');
        const response = await create_todo(title, content);
        console.log(response);
        setTodos(await get_todos())
        setAddTask(false);
    }

    const handleDragOver = (e) => {
        e.preventDefault();
        highlightIndicator(e);
    }

    const handleDragLeave = (e) => {
        e.preventDefault();
        clearHighlights();
    }

    const handleDragEnd = (e) => {
        e.preventDefault();
        clearHighlights();
    }

    const highlightIndicator = (e) => {
        const indicators = getIndicators();
        const el = getNearesIndicator(e, indicators);
        el.element.style.opacity = "1";
    }

    const clearHighlights = (els) => {
        const indicators = els || getIndicators();
        indicators.forEach((i) => {
            i.style.opacity = "0";
        })
    }

    const getIndicators = () => {
        const pending = Array.from(document.querySelectorAll("[data-column=Pending]"));
        const complete = Array.from(document.querySelectorAll("[data-column=Complete]"))
        return pending.concat(complete);
    }

    const getNearesIndicator = (e, indicators) => {
        const DISTANCE_OFFSET = 5;
        const el = indicators.reduce(
            (closest, child) => {
                const box = child.getBoundingClientRect();
                const offset = e.clientY - (box.top + DISTANCE_OFFSET);
                if (offset < 0 && offset > closest.offset) {
                    return { offset: offset, element: child };
                } else {
                    return closest;
                }
            }, {
            offset: Number.NEGATIVE_INFINITY,
            element: indicators[indicators.length - 1],
        }
        )
        return el;
    }

    const Burn = () => {
        const [active, setActive] = useState(false);

        const handleBurnOver = (e) => {
            e.preventDefault();
            setActive(true);
        }

        const handleBurnLeave = (e) => {
            e.preventDefault();
            setActive(false);
        }

        const handleBurnEnd = async (e) => {
            const cardId = e.dataTransfer.getData("cardId");
            const response = await delete_todo(cardId);
            console.log(response);
            setTodos(await get_todos());
            setActive(false);
        }

        return (
            <>
                <div onDragOver={handleBurnOver} onDragLeave={handleBurnLeave} onDrop={handleBurnEnd} className={`w-56 h-56 bg-neutral-700 mt-10 mr-10 place-content-center shrink-0 rounded border ${active ? "bg-red-800/20 text-red-500 border-red-800" : "border-neutral-500 bg-neutral-500/20 text-neutral-500"}`}>
                    <div className="w-full h-full flex justify-center items-center">
                        {active ? <img src={fireIcon} height={40} width={40} className="animate-bounce" /> : <img src={deleteIcon} height={40} width={40} />}

                    </div>
                </div>
            </>
        )
    }

    return (
        <>
            <div className="w-screen flex flex-row justify-between items-start">
                <div className="h-full">
                    <div className="w-full mt-5 ml-5"><h1 className="text-3xl text-white">Your Todos</h1></div>
                    {todos.length >= 0 ? <>
                        <div className="flex flex-col space-y-15" onDragLeave={handleDragLeave} onDragEnd={handleDragEnd} onDragOver={handleDragOver}>
                            <div className="flex flex-col mt-5 ml-5 gap-5">
                                <p className="text-md text-red-400">Pending</p>
                                <div className="space-y-2 w-full">
                                    {todos?.filter((item) => item.status === "incomplete").map((item, index) => (<Card key={index} item={item} index={index} column={"Pending"} />))}
                                    <DropIndicator before-id={"-1"} column={"Pending"} />
                                </div>
                            </div>
                            <div className="flex flex-col mt-5 ml-5">
                                <p className="text-md text-green-400">Complete</p>
                                <div className="space-y-2 w-full">
                                    {todos?.filter((item) => item.status === "complete").map((item, index) => (<Card key={index} item={item} index={index} column={"Complete"} />))}
                                    <DropIndicator before-id={"-1"} column={"Complete"} />
                                </div>
                            </div></div>
                    </> : <><p className="ml-5 mt-5 text-xl text-white">You have not created any todos yet!</p></>}
                    {addTask ? <></> : <><div className="mt-5 ml-5 text-textgray hover:text-white"><button onClick={(e) => toggleAdd(e)}>Add Task +</button></div></>}
                    {addTask ? <><div className="mt-5 ml-5 w-[30vw] h-32">
                        <textarea autoFocus className="h-[50%] w-[90%] bg-violet-400/20 border-violet-400 text-md text-white px-2 py-2 border rounded-md resize-none focus:outline-white focus:outline-0 placeholder-violet-400" placeholder="Add new task" onChange={(e) => setText(e.target.value)}></textarea>
                        <div className="w-[90%] pr-2 flex items-end justify-end gap-3">
                            <button className="text-textgray text-sm px-2 py-1 hover:text-white" onClick={() => setAddTask(false)}>Close</button>
                            <button className="text-black bg-white text-sm px-2 py-1 rounded-sm hover:brightness-75" onClick={handleSubmit}>Add +</button>
                        </div>
                    </div></> : <></>}
                </div>
                <Burn />
            </div>
        </>
    )
}

export default Board;

const handleDragStart = (e, card) => {
    e.dataTransfer.setData("cardId", card.item.id);
}

const Card = ({ item, index, column }) => {
    return (
        <>
            <DropIndicator before-id={index + 1} data-column={column} />
            <div draggable="true" onDragStart={(e) => handleDragStart(e, { item })} className="bg-notesbody border border-neutral-700 px-3 py-2 rounded-sm text-md text-white break-words active:cursor-grabbing">{index + 1}{". "}{item.title}</div>
        </>

    )
}



