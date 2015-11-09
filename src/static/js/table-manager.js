$(document).ready(function() {
	function get_cell_text($row) {
		return $row.find('td:first-child').text();
	}
	
	function matches($row, filter_val) {
		return get_cell_text($row).indexOf(filter_val) !== -1;
	}
	
	function filter_rows($rows, filter_val) {
		$rows.each(function() {
			var $row = $(this);
			if (matches($row, filter_val)) {
				$row.show();
			} else {
				$row.hide();
			}
		});
	}
	
	function get_street_name() {
		var text = get_cell_text($(this));
		var splitted = text.split(/[.|,]/);
		return (splitted[1] ? splitted[1] : text).trim();
	}
	
	function get_street_list($rows) {
		is_present = {};
		return $rows
			.map(get_street_name)
			.filter(function(_, val) {
				if (val in is_present) {
					return false;
				}
				is_present[val] = true;
				return true;})
			.sort();
	}
	
	function append_filter_button(name, filter_text, $rows) {
		$button = $('<button class="btn btn-default">' + name + '</button>');
		$button.click(function() {
			filter_rows($rows, filter_text);
		});
		$container.append($button);
	}
	
	var $container = $('#street-filter-container');
	var $rows = $('table :not(thead) tr');
	append_filter_button('Все', '', $rows);
	get_street_list($rows).each(function() {
		append_filter_button(this, this, $rows);
	});
});