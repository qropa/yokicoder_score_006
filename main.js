import __wbg_init, { vis, get_max_turn, edit_output } from './vis/pkg/vis.js';
__wbg_init().then(() => {});

const options = {
    type: 'openDirectory'
};

let selectedDirHandle = null;

async function fileselect() {
    selectedDirHandle = await window.showDirectoryPicker(options);
    updateInOut();
}
window.fileselect = fileselect;

async function updateInOut() {
    const seed = document.getElementById("seed").value;
    const file_name = `./in/${String(seed).padStart(4, '0')}.txt`;
    fetch(file_name)
        .then(response => response.text())
        .then(data => {
            document.getElementById("input").value = data;
        });
    updateResult();
    if ( selectedDirHandle == null ) { updateResult(); return; }
    try {
        const outfilename = `${String(seed).padStart(document.getElementById("zfill").value, '0')}.${document.getElementById("extension").value}`;
        const fileHandle = await selectedDirHandle.getFileHandle(outfilename);
        const file = await fileHandle.getFile();
        const text = await file.text();
        output.value = text;
    } catch(err) {
        output.value = '';
    }

    updateResult();
}
window.updateInOut = updateInOut;

function visualize() {
    const input = document.getElementById("input").value;
    const output = document.getElementById("output").value;
    const t = document.getElementById("turn").value;
    try {
        const res = vis(input, output, t);
        document.getElementById("score").innerHTML = "Score = " + res.score;
        document.getElementById("error").innerHTML = "Error = " + res.error;
        document.getElementById("level").innerHTML = "Level = " + res.level;
        document.getElementById("total_exp").innerHTML = "Total exp = " + res.total_exp;
        document.getElementById("result").innerHTML = res.svg;
    } catch (error) {
        document.getElementById("result").innerHTML = "<p>Invalid</p>";
    }
    setEvent();
}
window.visualize = visualize;

function update_t(t) {
    const max_turn = Number(document.getElementById("turn").max);
    const new_turn = Math.min(Math.max(0, t), max_turn);
    document.getElementById("turn").value = new_turn;
    document.getElementById("t_bar").value = new_turn;
    visualize();
}
window.update_t = update_t;
  
var prev = Date.now();
const play = document.getElementById("play");
const speed = document.getElementById("speed");
  
function start_autoplay() {
    if (Number(document.getElementById("turn").value) >= Number(document.getElementById("turn").max)) {
        document.getElementById("turn").value = 0;
    }
    prev = Date.now();
    play.value = "■";
    update_t(document.getElementById("turn").value);
}
window.start_autoplay = start_autoplay;

function updateResult() {
    play.value = "▶";
    const input = document.getElementById("input").value;
    const output = document.getElementById("output").value;
    try {
        const t = get_max_turn(input, output);
        document.getElementById("turn").max = t;
        document.getElementById("t_bar").max = t;
        update_t(t);
    } catch (error) {
        document.getElementById("result").innerHTML = "<p>Invalid</p>" + error;
    }
}
window.updateResult = updateResult;


  

play.onclick = event => {
    if (play.value == "■") {
        play.value = "▶";
    } else {
        start_autoplay();
    }
}

function autoplay() {
    if (play.value == "■") {
        const now = Date.now();
        let s = 2000;
        if ((now - prev) * speed.value >= s) {
            const inc = Math.floor((now - prev) * speed.value / s);
            prev += Math.floor(inc * s / speed.value);
            update_t(Number(document.getElementById("turn").value) + inc);
            if (Number(document.getElementById("turn").value) >= Number(document.getElementById("turn").max)) {
                play.value = "▶";
            }
        }
    }
    requestAnimationFrame(autoplay);
}
autoplay();

document.body.addEventListener('keydown',
    event => {
        if (document.getElementById("manual").checked) {
            const input = document.getElementById("input").value;
            const output = document.getElementById("output").value;
            const t = document.getElementById("turn").value;
            if (event.key == 'ArrowLeft') {
                document.getElementById("output").value = edit_output(input, output, t, 'L');
                updateResult();
                event.preventDefault();
            } else if (event.key == 'ArrowRight') {
                document.getElementById("output").value = edit_output(input, output, t, 'R');
                updateResult();
                event.preventDefault();
            } else if (event.key == 'ArrowUp') {
                document.getElementById("output").value = edit_output(input, output, t, 'S');
                updateResult();
                event.preventDefault();
            } else if (event.key == 'Backspace') {
                document.getElementById("output").value = edit_output(input, output, t, '-');
                updateResult();
                event.preventDefault();
            }
        }
    }
)

function setEvent() {
    let tooltip = document.getElementById("tooltip");

    for (let i = 0; i <= 1000; i++) {
        let el = document.getElementById("enemy" + i);
        if (el === null) {
            break;
        }
        console
        el.addEventListener("mouseover", function(event) {
            tooltip.innerHTML = el.getAttribute("title");
            tooltip.style.left = event.pageX + 10 + "px";
            tooltip.style.top = event.pageY - 70 + "px";
            tooltip.style.display = "block";
        });
        el.addEventListener("mouseout", function() {
            tooltip.style.display = "none";
        });
    }
}
window.setEvent = setEvent;
