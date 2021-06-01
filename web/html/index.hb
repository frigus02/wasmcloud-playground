<!DOCTYPE html>
<html>
<head>
	<title>wasmcloud Playground</title>
</head>
<body>
	<h1>Todos</h1>
	<ul>
		{{#each .}}
		<li>{{id}} {{title}} {{completed}}</li>
		{{/each}}
	</ul>
</body>
</html>
