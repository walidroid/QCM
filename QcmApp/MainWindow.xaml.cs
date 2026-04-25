using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Media.Animation;
using System.Windows.Media.Media3D;

namespace QcmApp
{
    public partial class MainWindow : Window
    {
        private List<QuestionData> _questions = new List<QuestionData>();
        private int _currentIndex = 0;
        private int _score = 0;
        private bool _isFinished = false;

        public MainWindow()
        {
            InitializeComponent();
            LoadQuestions();
            Start3DAnimation();
            ShowQuestion();
        }

        private void Start3DAnimation()
        {
            DoubleAnimation animation = new DoubleAnimation
            {
                From = 0,
                To = 360,
                Duration = new Duration(TimeSpan.FromSeconds(8)),
                RepeatBehavior = RepeatBehavior.Forever
            };
            CubeRotation.BeginAnimation(AxisAngleRotation3D.AngleProperty, animation);
        }

        private void LoadQuestions()
        {
            string filePath = "output.txt";
            if (!File.Exists(filePath))
            {
                MessageBox.Show($"Could not find the question file: {filePath}", "Error", MessageBoxButton.OK, MessageBoxImage.Error);
                return;
            }

            var lines = File.ReadAllLines(filePath).Where(l => !string.IsNullOrWhiteSpace(l)).ToList();
            
            QuestionData currentQ = null;
            
            int startIndex = 0;
            if (lines.Count > 0 && !lines[0].Contains("?")) startIndex = 1;
            if (lines.Count > 1 && !lines[1].Contains("?")) startIndex = 2;

            for (int i = startIndex; i < lines.Count; i++)
            {
                string line = lines[i].Trim();
                if (line.StartsWith("☐") || line.StartsWith("-"))
                {
                    if (currentQ != null)
                    {
                        string optionText = line;
                        if (optionText.StartsWith("☐")) optionText = optionText.Substring(1).Trim();
                        currentQ.Options.Add(optionText);
                    }
                }
                else
                {
                    if (currentQ != null)
                        _questions.Add(currentQ);
                    
                    currentQ = new QuestionData { Title = line, Options = new List<string>() };
                }
            }
            if (currentQ != null)
                _questions.Add(currentQ);
        }

        private void ShowQuestion()
        {
            OptionsPanel.Children.Clear();

            if (_currentIndex < _questions.Count)
            {
                var q = _questions[_currentIndex];
                QuestionTitle.Text = q.Title;
                
                foreach (var opt in q.Options)
                {
                    RadioButton rb = new RadioButton { Content = opt, GroupName = "Options" };
                    OptionsPanel.Children.Add(rb);
                }
                
                NextButton.Content = _currentIndex == _questions.Count - 1 ? "Finish" : "Next Question";
                ScoreText.Text = $"Question {_currentIndex + 1} of {_questions.Count}";
            }
            else
            {
                _isFinished = true;
                QuestionTitle.Text = "🎉 Quiz Completed! 🎉\nThank you for participating.";
                QuestionTitle.HorizontalAlignment = HorizontalAlignment.Center;
                QuestionTitle.TextAlignment = TextAlignment.Center;
                OptionsPanel.Visibility = Visibility.Collapsed;
                NextButton.Content = "Exit";
                ScoreText.Visibility = Visibility.Collapsed;
            }
        }

        private void NextButton_Click(object sender, RoutedEventArgs e)
        {
            if (_isFinished)
            {
                Application.Current.Shutdown();
                return;
            }

            bool answered = false;
            foreach (UIElement child in OptionsPanel.Children)
            {
                if (child is RadioButton rb && rb.IsChecked == true)
                {
                    answered = true;
                    _score++;
                    break;
                }
            }

            if (!answered)
            {
                MessageBox.Show("Please select an answer to proceed.", "Notice", MessageBoxButton.OK, MessageBoxImage.Information);
                return;
            }

            _currentIndex++;
            ShowQuestion();
        }
    }

    public class QuestionData
    {
        public string Title { get; set; }
        public List<string> Options { get; set; }
    }
}
