import Event from "../data/event/Event";

const events: Array<Event> = [
    {
        "type": 'homework',
        "name": 'Neuer Server einrichten',
        "start": 1626675900 * 1000,
        "end": 1626675900 * 1000,
        "description": 'Server mit Ubuntu einrichten'
    },
    {
        "type": 'exam',
        "name": 'Hennet Überraschungs Test 😔',
        "start": 1626675900 * 1000,
        "end": 1626675900 * 1000,
        "description": 'Server mit Ubuntu einrichten'
    },
    {
        "type": 'exam',
        "name": 'Mathe Prüfung',
        "start": 1626586500 * 1000,
        "description": 'Pythagoras dies das Ananas'
    },
    {
        "type": 'holidays',
        "name": 'Ferien',
        "start": 1627855200 * 1000,
        "end": 1629064799 * 1000,
        "description": 'Wisliger Casino V2.0? O_o :zeigarsch:'
    }
]

const getEvents = (): Array<Event> => {
    return events;
}

export default getEvents