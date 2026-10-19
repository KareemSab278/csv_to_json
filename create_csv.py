import csv

products = [
    [1, "Wireless Mouse", "Electronics", 24.99, 'true'],
    [2, "USB-C Hub", "Electronics", 39.50, 'true'],
    [3, "Notebook", "Stationery", 4.25, 'true'],
    [4, "Ballpoint Pens Pack of 10", "Stationery", 6.99, 'true'],
    [5, "Desk Lamp", "Home & Office", 32.00, 'false'],
    [6, "Water Bottle", "Accessories", 14.75, 'true'],
    [7, "Bluetooth Speaker", "Electronics", 49.99, 'true'],
    [8, "Canvas Tote Bag", "Accessories", 12.50, 'false'],
    [9, "Mechanical Keyboard", "Electronics", 79.95, 'true'],
    [10, "Ceramic Mug", "Home & Office", 8.50, 'true'],
]

with open("products.csv", "w", newline="", encoding="utf-8") as file:
    writer = csv.writer(file)

    writer.writerow([
        "product_id",
        "product_name",
        "product_category",
        "product_price",
        "product_availability",
    ])

    writer.writerows(products)

print("products.csv created successfully!")
