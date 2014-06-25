
//v Definition of the only type of object that a list can contain.
struct ListItem {
    value: TickType_t,		//< The value being listed.  In most cases this is used to sort the list in descending order.
    next: &ListItem,		//< Pointer to the next ListItem_t in the list.
    previous: &ListItem,	//< Pointer to the previous ListItem_t in the list.
    owner: &TaskControlBlock,	//< Pointer to the object (normally a TCB) that contains the list item.
    						//  There is therefore a two way link between the object containing the list item and the list item itself.
    container: &List		//< Pointer to the list in which this list item is placed (if any).
}

//v Definition of the type of queue used by the scheduler.
struct MiniListItem {
    value: TickType_t,
    next: &ListItem,
    previous: &ListItem
}

//v Definition of the type of queue used by the scheduler.
struct List {
    number_of_items: uint,
    index: &ListItem,		//< Used to walk through the list.  Points to the last item returned by a call to listGET_OWNER_OF_NEXT_ENTRY ().
    list_end: MiniListItem	//< List item that contains the maximum possible item value meaning it is always at the end of the list and is therefore used as a marker.
}

impl List {
	pub fn init(&self) {
		// The list structure contains a list item which is used to mark the
		// end of the list.  To initialise the list the list end is inserted
		// as the only list entry.
		self.index = &self.list_end

		// The list end value is the highest possible value in the list to
		// ensure it remains at the end of the list.
		self.list_end.value = TickType_MaxDelay

		// The list_end next and previous pointers point to itself so we know
		// when the list is empty.
		self.list_end.next = &self.list_end
		self.list_end.previous = &self.list_end

		self.number_of_items = 0;
	}

	pub fn insert_end(&self, new_item: &ListItem) {
		let old_index = self.index
		/* Insert a new list item into pxList, but rather than sort the list,
		   makes the new list item the last item to be removed by a call to
		   listGET_OWNER_OF_NEXT_ENTRY(). */
		new_item.next = old_index
		new_item.previous = old_index.previous
		old_index.previous.next = new_item
		old_index.previous = new_item

		new_item.container = self

		self.number_of_items++;
	}

	//v Insert the new list item into the list, sorted in value order.
	pub fn insert(&self, new_item: &ListItem) {
		ListItem_t &iterator =
			if new_item.value == TickType_MaxDelay {
				self.list_end.previous
			} else {
				/* If you find your application is crashing here then likely causes are
				   1) Stack overflow
				   2) Incorrect interrupt priority assignment
				   3) Calling an API function from within a critical section
				   4) Using a queue or semaphore before it has been initialised */
				sub_iterator = & self.list_end
				while sub_iterator.next.value <= new_item.value {
					sub_iterator = sub_iterator.next
				}
				sub_iterator
			}
		new_item.next = iterator.next
		new_item.next.previous = new_item
		new_item.previous = iterator
		iterator.next = new_item

		new_item.container = self

		self.number_of_items++;
	}

	//v Advance index, wrapping around if at end, and return owner
	pub fn owner_of_next(&self) -> TaskControlBlock {
		self.index = self.index.next
		if self.index == &(self.index.list_end) {
			self.index = self.index.next
		}
		return self.index.owner
	}

	pub fn owner_of_head(&self) -> TaskControlBlock {
		return self.list_end.next.owner
	}

	pub fn contains(&self, item: &ListItem) -> bool {
		return self == item.container
	}
}

impl ListItem {
	pub fn remove(&self) -> uint {
		self.next.previous = self.previous
		self.previous.next = self.next
		// Make sure the index is left pointing to a valid item.
		if (self.container.index == self) {
			self.container.index = self.previous
		} else {
			mtCOVERAGE_TEST_MARKER();
		}

		let new_num = self.container.number_of_items--
		self.container = NULL // ERROR
		new_num
	}
}









