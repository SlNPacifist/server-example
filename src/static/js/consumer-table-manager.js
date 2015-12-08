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
		{selector: 'table th:nth-child(3)', key: get_date_sort_key},
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
		var res = get_street_text($row).split(/[.|,]/).slice(1).map(function(val) {
			return val.trim();
		});
		var full_house_num = res[res.length - 1];
		var house_num = parseInt(full_house_num);
		if ((house_num + '').length < full_house_num.length) {
			res[res.length - 1] = house_num;
			var leftover = full_house_num.slice((house_num + '').length).replace(/^\-/gm, '');
			res.push(leftover);
		}
		return res;
	}
	
	function get_date_sort_key($row) {
		var text = get_date_text($row);
		var m = text.match(/(\d{4})-(\d{2})-(\d{2})/);
		if (m) return m.slice(1);
		return ["0", get_street_text($row)];
	}
	
	function apply_sort_order($rows) {
		$rows.each(function() {
			this.parentNode.appendChild(this);
		});
	}
	
	function compare_arrays(a, b) {
		var l = Math.min(a.length, b.length);
		for (var i = 0; i < l; i++) {
			var diff = a[i] - b[i];
			if (!isNaN(diff) && diff != 0) return diff;
			if (a[i] < b[i]) {
				return -1;
			} else if (a[i] > b[i]) {
				return 1;
			}
		}
		if (a.length < b.length) {
			return -1;
		} else if (a.length > b.length) {
			return 1;
		}
		return 0;
	}
	
	function sort_by($rows, key, is_desc) {
		$rows.sort(function(a, b) {
			key_a = key($(a));
			key_b = key($(b));
			var res = compare_arrays(key_a, key_b);
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
	
	function revert_sort_order(column, $rows) {
		var state = (column.state == 'asc') ? 'desc' : 'asc';
		drop_all_sort_states();
		sort_by($rows, column.key, state == 'desc');
		apply_sort_order($rows);
		update_column_state(column, state);
	}
	
	function create_column_sort_button(column, $rows) {
		column.$switcher = $('<span class="glyphicon">');
		update_column_state(column, 'non-sorted');
		var $header = $(column.selector);
		$header.append(column.$switcher);
		$header.wrapInner('<button type="button" class="btn btn-default"></button>');
		function rev_sort() {
			revert_sort_order(column, $rows);
		}
		$($header[0].childNodes[0]).click(rev_sort);
	}

	var $rows = $('table :not(thead) tr');
	create_street_filters($rows);
	$(sorted_columns).each(function() {
		create_column_sort_button(this, $rows)
	});
	revert_sort_order(sorted_columns[0], $rows);
});