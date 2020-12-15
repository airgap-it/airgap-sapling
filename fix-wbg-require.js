const replace = require('replace-in-file');
const fs = require('fs').promises;

(async () => {
    const files = await fs.readdir('./dist')
    const options = {
        files: files.filter((file) => file.endsWith(".js")).map((file) => 'dist/' + file),
        from: /getObject\(arg0\).require\(getStringFromWasm0\(arg1, arg2\)\)/g,
        to: 'require(getStringFromWasm0(arg1, arg2))'
    }
    replace(options)
})() 