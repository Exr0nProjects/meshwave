const PORT = 3260;

const express = require('express');
const app = express();

app.get('/', (_, res) => {
    res.sendFile(__dirname + '/dist/index.html');
});

app.use(express.static(__dirname + '/dist'));

app.listen(PORT, () => { console.log(`Listening on port ${PORT}`) });

