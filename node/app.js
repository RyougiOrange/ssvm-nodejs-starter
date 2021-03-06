const express = require('express');
const { getpi } = require('../pkg/ssvm_nodejs_starter_lib.js');

const app = express();
const port = 3000;
app.use(express.static(__dirname + '/public'));
app.use(express.urlencoded({ extended: false }));

app.get('/', (req, res) => res.redirect("/index.html"));

app.post('/getpi', (req, res) => {
  // console.log(req)
  // console.log(req.body.inputnum)
  let pinum = parseInt(req.body.inputnum);
  // console.log(pinum)
  res.send(getpi(pinum))
})

app.listen(port, () => console.log(`Listening at http://localhost:${port}`))