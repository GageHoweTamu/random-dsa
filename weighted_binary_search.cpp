int weighted_binary_search(long long arr[], int size, int target,
                           double weight = 0.5) {
  int left = 0;
  int right = size - 1;

  while (left <= right) {
    int mid = left + static_cast<int>((right - left) * weight);
    if (arr[mid] == target) {
      return mid;
    } else if (arr[mid] < target) {
      left = mid + 1;
    } else {
      right = mid - 1;
    }
  }
  return -1;
}
