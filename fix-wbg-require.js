const replace = require('replace-in-file');
const fs = require('fs').promises;

(async () => {
    const files = await fs.readdir('./dist')
    const options = {
        files: files.filter((file) => file.endsWith(".js")).map((file) => 'dist/' + file),
        from: /\w+\(\w+\).require\((\w+)\((\w+),\s*(\w+)\)\)/g,
        to: 'require($1($2,$3))'
    }
    replace(options)
})() 