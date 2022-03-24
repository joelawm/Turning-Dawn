using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class GameManager : MonoBehaviour
{
	private void Awake()
	{
		if(instance == null) 
		{
			instance = this;
		}
		else if (instance != this) 
		{
			Debug.Log("Instance already exists, destroying object");
			Destory(this);
		}
	}
}
