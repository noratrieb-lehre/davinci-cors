import TimeTable from "../data/timetable/TimeTable";

const timetable: TimeTable = [
    [],
    [
        {
            subject: "Mathematik",
            description: "Benz beste",
            start: 20_100_000,
            end: 28_800_000
        },
        {
            subject: "Chemie",
            description: "Benz beste",
            start: 30_000_000,
            end: 35_700_000
        },
        {
            subject: "Französisch",
            description: "Benz beste",
            start: 39_300_000,
            end: 45_000_000
        },
        {
            subject: "Englisch",
            description: "Benz beste",
            start: 45_300_000,
            end: 51_600_000
        },
    ],
    [],
    [],
    [],
    [],
    []
];

const getTimeTable = () => {
    return timetable;
}

export default getTimeTable;