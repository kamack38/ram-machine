const fs = require('fs')
const filePath = './Cargo.lock'

exports.preCommit = (props) => {
  const tomlContent = fs.readFileSync(filePath, 'utf8')
  fs.writeFileSync(filePath, tomlContent.replace(/(name = "ram-machine"[\s\S]version = )".*"/m, `$1"${props.tag}"`))
}
