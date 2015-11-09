$(document).ready(function() {
	var sort_glyph_classes = {
		'asc': 'glyphicon-sort-by-alphabet',
		'desc': 'glyphicon-sort-by-alphabet-alt',
		'non-sorted': 'glyphicon-sort',
	}
	var all_sort_glyph_classes = Object.keys(sort_glyph_classes)
		.map(function(k) {return sort_glyph_classes[k]})
		.join(' ');
	var sorted_columns = [
		{selector: 'table th:first-child', key: get_street_sort_key},
		{selector: 'table th:nth-child(3)', key: get_street_text},
	];
	
	function get_street_text($row) {
		return $row.find('td:first-child').text();
	}
	
	function get_date_text($row) {
		return $row.find('td:nth-child(3)').text();
	}
	
	function matches($row, filter_val) {
		return get_street_text($row).indexOf(filter_val) !== -1;
	}
	
	function filter_rows($rows, filter_val) {
		$rows.each(function() {
			var $row = $(this);
			if (matches($row, filter_val)) {
				$row.removeClass('hidden');
			} else {
				$row.addClass('hidden');
			}
		});
	}
	
	function get_street_name() {
		var text = get_street_text($(this));
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
	
	function get_filter_button(name, filter_text, $rows) {
		$button = $('<button class="btn btn-default">' + name + '</button>');
		$button.click(function() {
			filter_rows($rows, filter_text);
		});
		return $button;
	}
	
	function get_street_sort_key($row) {
		var res = get_street_text($row).split(/[.|,]/);
		if (2 in res) {
			res[2] = parseInt(res[2]);
		}
		return res;
	}
	
	function apply_sort_order($rows) {
		$rows.each(function() {
			this.parentNode.appendChild(this);
		});
	}
	
	function sort_by($rows, key, is_desc) {
		$rows.sort(function(a, b) {
			key_a = key($(a));
			key_b = key($(b));
			res = 0;
			if (key_a < key_b) {
				res = -1
			} else if (key_a > key_b) {
				 res = 1;
			}
			if (is_desc) res = -res;
			return res;
		});
		return $rows;
	}
	
	function create_street_filters($rows) {
		var $container = $('#street-filter-container');
		$container.append(get_filter_button('Все', '', $rows));
		get_street_list($rows).each(function() {
			$container.append(get_filter_button(this, this, $rows));
		});
	}
	
	function update_column_state(column, state) {
		column.state = state;
		column.$switcher
			.removeClass(all_sort_glyph_classes)
			.addClass(sort_glyph_classes[state]);
	}
	
	function drop_all_sort_states() {
		$(sorted_columns).each(function() {
			update_column_state(this, 'non-sorted');
		});
	}
	
	function create_column_sort_button(column, $rows) {
		column.$switcher = $('<span class="glyphicon">');
		update_column_state(column, 'non-sorted');
		var $header = $(column.selector);
		$header.append(column.$switcher);
		$header.wrapInner('<button type="button" class="btn btn-default"></button>');
		function rev_sort() {
			var state = (column.state == 'asc') ? 'desc' : 'asc';
			drop_all_sort_states();
			sort_by($rows, column.key, state == 'desc');
			apply_sort_order($rows);
			update_column_state(column, state);
		}
		$($header[0].childNodes[0]).click(rev_sort);
	}

	var $rows = $('table :not(thead) tr');
	create_street_filters($rows);
	$(sorted_columns).each(function() {
		create_column_sort_button(this, $rows)
	});
});