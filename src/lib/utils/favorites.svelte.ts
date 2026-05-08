const STORAGE_KEY = "favorites";

export interface FavoriteItem {
	title: string;
	id: number;
}

class FavoritesStore {
	#items = $state<FavoriteItem[]>([]);

	constructor() {
		if (typeof window !== "undefined") {
			try {
				const raw = localStorage.getItem(STORAGE_KEY);
				if (raw) {
					const parsed = JSON.parse(raw) as unknown;
					if (Array.isArray(parsed)) {
						this.#items = parsed as FavoriteItem[];
					}
				}
			} catch {
				this.#items = [];
			}
		}

		$effect.root(() => {
			$effect(() => {
				localStorage.setItem(STORAGE_KEY, JSON.stringify(this.#items));
			});
		});
	}

	get items() {
		return this.#items;
	}

	get isEmpty() {
		return this.#items.length === 0;
	}

	add(item: FavoriteItem) {
		if (!this.#items.some((i) => i.id === item.id)) {
			this.#items.push(item);
		}
	}

	addById(title: string, id: number) {
		this.add({ title, id });
	}

	remove(id: number) {
		this.#items = this.#items.filter((i) => i.id !== id);
	}

	includes(id: number) {
		return this.#items.some((i) => i.id === id);
	}

	getTitle(id: number): string | undefined {
		return this.#items.find((i) => i.id === id)?.title;
	}

	clear() {
		this.#items = [];
	}
}

export const favorites = new FavoritesStore();
