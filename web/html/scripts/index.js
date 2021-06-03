document
	.querySelector(".new-todo")
	.addEventListener("keydown", async (event) => {
		if (event.key !== "Enter") return;

		const title = event.target.value;
		if (!title.trim()) return;

		const res = await fetch("/api", {
			method: "POST",
			body: JSON.stringify({ title }),
		});
		if (res.ok) {
			location = "/";
		} else {
			alert("Todo creation failed");
		}
	});

document
	.querySelector(".todo-list")
	.addEventListener("click", async (event) => {
		if (!event.target.matches(".toggle")) return;
		const id = event.target.closest("[data-id]").dataset.id;
		const res = await fetch(`/api/${id}`, {
			method: "PUT",
		});
		if (res.ok) {
			location = "/";
		} else {
			alert("Todo completion failed");
		}
	});
