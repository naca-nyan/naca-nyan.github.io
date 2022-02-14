module.exports = {
    pages: {
        index: {
            entry: 'src/index.ts',
            template: 'public/index.html',
            filename: 'index.html'
        },
        converter: {
            entry: 'src/converter.ts',
            template: 'public/index.html',
            filename: 'converter/index.html'
        },
        bpm: {
            entry: 'src/bpm.ts',
            template: 'public/index.html',
            filename: 'bpm/index.html'
        },
    }
}