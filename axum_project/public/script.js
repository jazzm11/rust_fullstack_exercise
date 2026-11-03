async function fetchData() {
    try {
        const response = await fetch('http://127.0.0.1:3000/api/hello_world');

        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        const data = await response.json();

        const messageElement = document.getElementById('message');
        messageElement.textContent = data.message;

        const titleElement = document.getElementById('title');
        titleElement.textContent = data.title;

    } catch (error) {
        console.error('Error fetching data:', error);
    }
}

document.addEventListener('DOMContentLoaded', () => {
    fetchData();
});