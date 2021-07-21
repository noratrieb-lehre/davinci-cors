import Event from "../data/event/Event";

const events: Array<Event> = [
    {
        "type": 'homework',
        "name": 'Neuer Server einrichten',
        "start": 1626675900,
        "description": 'Server mit Ubuntu einrichten'
    },
    {
        "type": 'exam',
        "name": 'Mathe Prüfung',
        "start": 1626586500,
        "description": 'Pythagoras dies das Ananas'
    },
    {
        "type": 'holidays',
        "name": 'Ferien',
        "start": 1627855200,
        "end": 1629064799,
        "description": 'Wisliger Casino V2.0? O_o :zeigarsch:'
    }
]

const getEvents = (): Array<Event> => {
    return events;
}

export default getEvents