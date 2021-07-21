import Class from "../data/class/Class";

const classRepo: Array<Class> = [
    {
        "id": "hugo",
        "name": "Hugos Flugklasse",
        "members": [
            {
                "user": "hugo",
                "displayName": "Hugo Boss",
                "class": "hugo",
                "role": "owner"
            }
        ],
        "description": 'Hugo hat eine Flugklasse'
    },
    {
        "id": "bbw",
        "name": "5IA20a",
        "members": [
            {
                "user": "luigi",
                "displayName": "Luigi Cavouti",
                "class": "bbw",
                "role": "owner"
            },
            {
                "user": "hennet",
                "displayName": "Michel Hennet",
                "class": "bbw",
                "role": "admin"
            },
            {
                "user": "hugo",
                "displayName": "Hugo Boss",
                "class": "bbw",
                "role": "member"
            },
            {
                "user": "corsi",
                "displayName": "Corsin Ragettli",
                "class": "bbw",
                "role": "member"
            }
        ],
        "description": 'Die beste Klasse '
    },
    {
        "name": '2j',
        "id": 'BMS',
        "members": [],
        "description": "Auch die beste Klasse"
    }
]

const getClasses = (): Array<Class> => {
    return classRepo;
}

export default getClasses