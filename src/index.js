//FOSS
const start = +process.argv[2] || 69;
const end = +process.argv[3] || 420;

function random() {
    return Math.floor(Math.random() * (end - start) + start);
}

console.log(random());

