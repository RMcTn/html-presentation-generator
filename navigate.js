window.addEventListener("keydown", function(e) {
	if (e.key == "ArrowLeft") {
		this.document.getElementById("previous").click();
	}
	if (e.key == "ArrowRight") {
		this.document.getElementById("next").click();
	}
});
