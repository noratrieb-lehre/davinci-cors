import Class from "../data/class/Class";

const classRepo: Array<Class> = [
    {
        "id": "hugo",
        "name": "Hugos Flugklasse",
        "owner": "Hugo Boss",
        "members": [],
        "description": 'Hugo hat eine Flugklasse'
    },
    {
        "id": "bbw",
        "name": "5IA20a",
        "owner": "Luigi Cavouti",
        "members": [],
        "description": 'Die beste Klasse '
    },
    {
        "name": '2j',
        "id": 'BMS',
        "owner": "Christoph Benz",
        "members": [],
        "description": "Auch die beste Klasse"
    }
]

const getClasses = (): Array<Class> => {
    return classRepo;
}

export default getClasses