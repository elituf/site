const quotes = [
    "who are you",
    "lorem ipsum dolor sit amet",
    "hot garfields in your area",
    "remember to drink water",
    "i am not a web developer",
    "it's nice, isn't it?",
    "do you like hurting other people?",
    "check out increm.net !",
    "get real",
];
document.addEventListener('DOMContentLoaded', () => {
    const randomIndex = Math.floor(Math.random() * quotes.length);
    document.getElementById('quote').textContent = '« ' + quotes[randomIndex] + ' »';
});

fetch('static/buttons/_alts.json')
    .then(response => response.json())
    .then(alts => {
        document.querySelectorAll("div.footer.buttons img").forEach(img => {
            const filename = img.src.split('/').pop();
            img.alt = alts[filename] || "no alt text available, sorry";
        });
    })
    .catch(error => console.error('error loading alt texts: ', error));
