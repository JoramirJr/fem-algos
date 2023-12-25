import { container } from "./container";

function linear_search() {
    const search: string = process.argv[2].toString();

    return (!!container.find(n => n.toString() === search) || false);
}

console.log(linear_search());