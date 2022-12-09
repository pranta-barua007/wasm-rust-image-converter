async function init() {
    let rustApp = null;

    try {
        rustApp = await import('../pkg');
        console.log(rustApp)
    }catch(err) {
        console.error(err);
        return
    }

    const input = document.getElementById('upload');
    const fileReader = new FileReader();

    fileReader.onloadend = () => {
        const base64 = fileReader.result.replace(
            /^data:image\/(png|jpeg|jpg);base64,/,
            ''
        );
        let image_data_url = rustApp.grayscale(base64);
            document.getElementById('new-img').setAttribute(
               'src', image_data_url 
            );
    }

    input.addEventListener('change', () => {
        fileReader.readAsDataURL(input.files[0]);
    })
}

init();